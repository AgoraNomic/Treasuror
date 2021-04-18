pub mod common;
pub mod gsdl;
pub mod tll;

#[macro_export]
macro_rules! match_first_pop {
    ($v:ident { $( $t:pat => $b:block ),+, } else $e:block) => {{
        // println!("{}", $v.len());
        let tmp_first = $v.get(0).cloned();
        match tmp_first {
            $(Some($t) => {
                $v.remove(0);
                $b
            },)*
            Some(_) | None => $e,
        }
    }}
}
