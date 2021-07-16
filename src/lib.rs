// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//
// escafers - parser & generator for escape sequences (external whitespace sequence support (\w))
//
// github.com/ferhatgec/escafers
// github.com/ferhatgec/escafe
// github.com/ferhatgec/escafe.py

pub mod escafe {
    pub fn run(data: &String, use_unsupported_seq: bool) -> String {
        let mut __data = String::new();
        let mut collect= String::new();
        let (mut is_escape, mut is_ansi) = (false, false);

        for ch in data.chars() {
            if !is_ansi && !collect.is_empty() {
                //match collect.as_str() {
                if collect == "x1b" { // => {
                    __data.push_str("\x1b");
                } //, _ => {}

                collect.clear();
            }

            if is_escape {
                match ch {
                    '\'' => {
                        __data.push('\''); is_escape = false;
                    },
                    '"' => {
                        __data.push('\"'); is_escape = false;
                    },
                    '\\' => {
                        __data.push('\\'); is_escape = false;
                    },
                    'a' => {
                        if use_unsupported_seq {
                            __data.push_str("\\a");
                        } is_escape = false;
                    },
                    'b' => {
                        if is_ansi {
                            collect.push('b');
                            is_ansi = false;
                        } else {
                            if use_unsupported_seq {
                                __data.push_str("\\b");
                            }
                        } is_escape = false;
                    },
                    'f' => {
                        if use_unsupported_seq {
                            __data.push_str("\\f");
                        } is_escape = false;
                    },
                    'n' => {
                        __data.push('\n'); is_escape = false;
                    },
                    'r' => {
                        __data.push('\r'); is_escape = false;
                    },
                    't' => {
                        __data.push('\t'); is_escape = false;
                    },
                    'v' => {
                        if use_unsupported_seq {
                            __data.push_str("\\v");
                        } is_escape = false;
                    },
                    'w' => {
                        __data.push(' '); is_escape = false;
                    },
                    'x' => {
                        is_ansi = true; collect.push('x');
                    },
                    '1' => {
                        if is_ansi {
                            collect.push('1');
                        }
                    }
                    _ => {
                        is_escape = false;
                    }
                } continue;
            }

            if ch == '\\' {
                is_escape = true; continue;
            } __data.push(ch);
        }

        __data
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        let data = String::from("escafe\\wis\\winteresting\\n\\x1b[0;33mtest\\x1b[0m\\n\\\\");
        //                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                                raw data taken from github.com/ferhatgec/escafe/

        println!("{}", super::escafe::run(&data, false));
    }
}
