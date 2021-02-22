use super::cssbuf::CssBuf;
use crate::css::{BodyItem, Rule};
use crate::error::Error;
use crate::file_context::FileContext;
use crate::parser::parse_imported_scss_file;
use crate::sass::{FormalArgs, Item};
use crate::selectors::Selectors;
use crate::value::ValueRange;
use crate::variablescope::{Scope, ScopeImpl};
use std::io::Write;

pub fn handle_body(
    items: &[Item],
    head: &mut CssBuf,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: &mut dyn Scope,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let mut rule = rule;
    for b in items {
        handle_item(b, head, rule.as_deref_mut(), buf, scope, file_context)?;
    }
    Ok(())
}

fn handle_item(
    item: &Item,
    head: &mut CssBuf,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: &mut dyn Scope,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let format = scope.get_format();
    match item {
        Item::Use(ref name, ref as_n) => {
            use crate::functions::get_global_module;
            use crate::sass::UseAs;
            let name = name.evaluate(scope)?.0;
            if let Some(module) = get_global_module(&name) {
                match as_n {
                    UseAs::KeepName => {
                        let name = name
                            .rfind(':')
                            .map(|i| &name[i + 1..])
                            .unwrap_or(&name);
                        scope.define_module(name.into(), module);
                    }
                    UseAs::Star => {
                        for (name, function) in module.functions() {
                            scope.define_function(
                                name.clone(),
                                function.clone(),
                            );
                        }
                        for (name, value) in module.variables() {
                            scope.define(name.clone(), value);
                        }
                    }
                    UseAs::Name(name) => {
                        let name = name.evaluate(scope)?.0;
                        scope.define_module(name.into(), module);
                    }
                }
            } else {
                return Err(Error::S(format!("Module {:?} not found", name)));
            }
        }
        Item::Import(ref names, ref args, ref pos) => {
            let mut rule = rule;
            'name: for name in names {
                if args.is_null() {
                    let (x, _q) = name.evaluate(scope)?;
                    if let Some((sub_context, path, mut file)) =
                        file_context.find_file(&x)?
                    {
                        let items = parse_imported_scss_file(
                            &mut file,
                            &path,
                            pos.clone(),
                        )?;
                        handle_body(
                            &items,
                            head,
                            rule.as_deref_mut(),
                            buf,
                            scope,
                            &sub_context,
                        )?;
                        continue 'name;
                    }
                }
                if buf.is_root_level() {
                    head.add_import(
                        name.evaluate2(scope)?,
                        args.evaluate(scope)?,
                    )?;
                } else {
                    buf.add_import(
                        name.evaluate2(scope)?,
                        args.evaluate(scope)?,
                    )?;
                }
            }
        }
        Item::AtRoot(ref selectors, ref body) => {
            let selectors = selectors
                .eval(scope)?
                .with_backref(scope.get_selectors().one());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                &mut ScopeImpl::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule, true)?;
            buf.join(sub);
        }
        Item::AtRule {
            ref name,
            ref args,
            ref body,
        } => {
            buf.do_separate();
            write!(buf, "@{}", name)?;
            let args = args.evaluate(scope)?;
            if !args.is_null() {
                write!(buf, " {}", args.format(format))?;
            }
            if let Some(ref body) = *body {
                buf.add_one(" {", "{");
                let selectors = scope.get_selectors().clone();
                let has_selectors = selectors != Selectors::root();
                let mut rule = Rule::new(selectors);
                let mut sub = CssBuf::new_below(buf);
                handle_body(
                    body,
                    head,
                    Some(&mut rule),
                    &mut sub,
                    &mut ScopeImpl::sub(scope),
                    file_context,
                )?;
                let mut t = CssBuf::new_as(&sub);
                if has_selectors {
                    t.write_rule(&rule, false)?;
                } else {
                    t.write_body_items(&rule.body)?;
                };
                if !t.is_empty() || !sub.is_empty() {
                    buf.join(t);
                    buf.do_indent();
                    buf.join(sub);
                }
                buf.add_str("}");
                buf.do_indent();
            } else {
                buf.add_str(";");
            }
        }

        Item::VariableDeclaration {
            ref name,
            ref val,
            default,
            global,
        } => {
            let val = val.do_evaluate(scope, true)?;
            scope.set_variable(name.into(), val, *default, *global);
        }

        Item::FunctionDeclaration(ref name, ref func) => {
            scope.define_function(name.into(), func.clone());
        }
        Item::Return(_) => {
            return Err(Error::S(
                "Return not allowed in plain context".into(),
            ));
        }

        Item::MixinDeclaration(ref name, ref args, ref body) => {
            scope.define_mixin(name, args, body)
        }
        Item::MixinCall(ref name, ref args, ref body) => {
            if let Some((m_args, m_body)) = scope.get_mixin(name) {
                let mut scope =
                    m_args.eval(scope, &args.evaluate(scope, true)?)?;
                scope.define_mixin("%%BODY%%", &FormalArgs::default(), body);
                handle_body(
                    &m_body,
                    head,
                    rule,
                    buf,
                    &mut scope,
                    file_context,
                )?;
            } else {
                return Err(Error::S(format!(
                    "Unknown mixin {}({:?})",
                    name, args
                )));
            }
        }
        Item::Content => {
            if let Some(rule) = rule {
                if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%") {
                    handle_body(
                        &m_body,
                        head,
                        Some(rule),
                        buf,
                        scope,
                        file_context,
                    )?;
                }
            } else {
                return Err(Error::S(
                    "@content not allowed in global context".into(),
                ));
            }
        }

        Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope)?.is_true();
            let items = if cond { do_if } else { do_else };
            handle_body(items, head, rule, buf, scope, file_context)?;
        }
        Item::Each(ref names, ref values, ref body) => {
            let mut rule = rule;
            let pushed = scope.store_local_values(names);
            for value in values.evaluate(scope)?.iter_items() {
                scope.define_multi(&names, &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope,
                    file_context,
                )?;
            }
            scope.restore_local_values(pushed);
        }
        Item::For {
            ref name,
            ref from,
            ref to,
            inclusive,
            ref body,
        } => {
            let range = ValueRange::new(
                from.evaluate(scope)?,
                to.evaluate(scope)?,
                *inclusive,
            )?;
            let mut rule = rule;
            for value in range {
                let mut scope = ScopeImpl::sub(scope);
                scope.define(name.clone(), &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    &mut scope,
                    file_context,
                )?;
            }
        }
        Item::While(ref cond, ref body) => {
            let mut rule = rule;
            let mut scope = ScopeImpl::sub(scope);
            while cond.evaluate(&scope)?.is_true() {
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    &mut scope,
                    file_context,
                )?;
            }
        }

        Item::Debug(ref value) => {
            eprintln!("DEBUG: {}", value.evaluate(scope)?.format(format));
        }
        Item::Warn(ref value) => {
            eprintln!("WARNING: {}", value.evaluate(scope)?.format(format));
        }
        Item::Error(ref value) => {
            return Err(Error::S(format!(
                "Error: {}",
                value.evaluate(scope)?.format(format)
            )));
        }

        Item::Rule(ref selectors, ref body) => {
            if rule.is_none() {
                buf.do_separate();
            }
            let selectors =
                selectors.eval(scope)?.inside(scope.get_selectors());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                &mut ScopeImpl::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule, true)?;
            buf.join(sub);
        }
        Item::Property(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope)?;
                if !v.is_null() {
                    let (name, _q) = name.evaluate(scope)?;
                    rule.push(BodyItem::Property(name, v));
                }
            } else {
                return Err(Error::S("Global property not allowed".into()));
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            if let Some(rule) = rule {
                let value = value.evaluate(scope)?;
                let (name, _quotes) = name.evaluate(scope)?;
                if !value.is_null() {
                    rule.push(BodyItem::Property(name.clone(), value));
                }
                let mut t = Rule::new(Selectors::root());
                let mut sub = CssBuf::new(format);
                handle_body(
                    body,
                    head,
                    Some(&mut t),
                    &mut sub,
                    scope,
                    file_context,
                )?;
                for item in t.body {
                    rule.push(match item {
                        BodyItem::Property(n, v) => {
                            BodyItem::Property(format!("{}-{}", name, n), v)
                        }
                        c => c,
                    })
                }
                assert!(sub.is_empty());
            } else {
                return Err(Error::S(
                    "Global namespaced property not allowed".into(),
                ));
            }
        }
        Item::Comment(ref c) => {
            if !format.is_compressed() {
                if let Some(rule) = rule {
                    let (c, _q) = c.evaluate(scope)?;
                    rule.push(BodyItem::Comment(c));
                } else {
                    buf.do_separate();
                    let (c, _q) = c.evaluate(scope)?;
                    write!(buf, "/*{}*/", c)?;
                }
            }
        }

        Item::None => (),
    }
    Ok(())
}