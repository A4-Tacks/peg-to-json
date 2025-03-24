peg::parser!(pub grammar parser() for str {
    use json::JsonValue;

    rule trace<T>(r: rule<T>) -> T
        = #{|input, pos| {
            #[cfg(feature = "trace")] {
                print!("[PEG_INPUT_START]");
                if pos != 0 { print!(" from {pos}") }
                println!("\n{input}\n[PEG_TRACE_START]");
            }
            peg::RuleResult::Matched(pos, ())
        }}
        r:r()? {?
            #[cfg(feature = "trace")]
            println!("[PEG_TRACE_STOP]");
            r.ok_or("")
        }

    rule comment() = quiet!{";" [^'\n']* "\n"} / expected!("comment")
    rule _() = quiet!{[' '|'\t'|'\r'|'\n']*} (comment() _)?
    rule ident() -> String
        = s:$(quiet!{
            !['0'..='9'] ['0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '_']+
        }) { s.into() }
        / expected!("ident")
    rule number() -> u32
        = s:$(quiet!{ ['0'..='9']+ }) { s.parse().unwrap() }
        / expected!("number")
    rule string() -> String
        = quiet!{
            "\"" s:("\\\"" {"\""} / $([^'"' | '\r' | '\n']))* "\""
            { s.concat() }
        } / expected!("string")
    rule repeat() -> (u32, Option<Option<u32>>)
        = "+" { (1, Some(None)) }
        / "*" to:number()? { (0, Some(to)) }
        / base:number() r:("*" to:number()? { to })? { (base, r) }

    rule patatom() -> JsonValue
        = i:ident() !(_ "=") { i.into() }
        / s:string() { json::object! {match: s} }
        / "[" _ c:patchoice() _ "]" { json::object! {optional: c} }
        / "(" _ c:patchoice() _ ")" { c }
        / "()" { json::array![] }
    rule patops() -> JsonValue
        = r:repeat() _ p:patatom() {
            let (base, to) = r;
            let mut obj = json::object! {repeat: p};
            if base != 0 { obj.insert("base", base).unwrap(); }
            match to {
                Some(Some(to)) => obj.insert("to", to).unwrap(),
                Some(None) => obj.insert("to", true).unwrap(),
                None => (),
            }
            obj
        }
        / "&" p:patatom() { json::object! { look: p } }
        / "!" p:patatom() { json::object! { look: p, invert: true } }
        / patatom()
    rule patlist() -> JsonValue
        = ops:patops() ++ _
        {
            if ops.len() == 1 {
                ops.into_iter().next().unwrap()
            } else {
                JsonValue::Array(ops)
            }
        }
    rule patchoice() -> JsonValue
        = ops:patlist() ++ (_ "/" _)
        {
            if ops.len() == 1 {
                ops.into_iter().next().unwrap()
            } else {
                json::object! {
                    choice: JsonValue::Array(ops)
                }
            }
        }
    rule decl() -> (String, JsonValue)
        = k:ident() _ "=" _ c:patchoice() { (k, c) }
    pub
    rule decl_list() -> JsonValue
        = _ decls:trace(<decl()**_>) _
        {
            json::object::Object::from_iter(decls).into()
        }
});
