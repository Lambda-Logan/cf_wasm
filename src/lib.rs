use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_ftzrs(a: usize, b: usize, your_string: String, your_doc: String) -> String {
    let mut lines: Vec<String> = Vec::new(); //vec![string, sentence, doc];

    let start_block = |v: &mut Vec<String>| {
        v.push("<p style=\"background-color:lightgoldenrodyellow;\">".to_owned());
    };
    let end_block = |v: &mut Vec<String>| {
        v.push("</p>".to_owned());
    };

    macro_rules! println {
        () => {//lines.push("<br></br>".to_owned());
                end_block(&mut lines);
                start_block(&mut lines);

    };
        ($($arg:tt)*) => ({
            let pretty = format!($($arg)*);
            let mut s = "".to_owned();
            if pretty.starts_with(">") {
                s.push_str("<br>");
            } else {
                s.push_str("<br style=\"background-color:lightgray;\">");
            }
            s.push_str(pretty.as_str());
            s.push_str("</br>");
            lines.push(s);
            //$crate::io::_print($crate::format_args_nl!($($arg)*));
        })
    }

    use creature_feature::convert::*;
    use creature_feature::ftzrs::*;
    use creature_feature::traits::*;
    use creature_feature::HashedAs;
    use std::collections::*;

    start_block(&mut lines);

    if your_string.len() >= a {
        println!(
            "let features: Vec<&str> = n_slice({:?}).featurize({:?});",
            a, your_string
        );
        let features: Vec<&str> = n_slice(a).featurize(&your_string);
        println!(">>> {:?}", features);
    }

    if your_string.len() >= b {
        println!();

        println!(
            "let features: LinkedList<[u8; {:?}]> = n_gram::<{:?}>().featurize({:?});",
            b, b, your_string
        );
        let features: LinkedList<&[u8]> = n_slice(b).featurize(&your_string);
        println!(">>> {:?}", features);
    }

    if your_string.len() >= (a + a + b) {
        println!();

        println!(
            "let ftzr = gap_gram(n_slice({:?}), {:?}, n_slice({:?}));",
            a, b, a
        );
        println!(
            "let features: BTreeSet<(&str, &str)> = ftzr.featurize({:?});",
            your_string
        );
    }

    println!(">>> {:?}", {
        let ftzr = gap_gram(n_slice(a), b, n_slice(a));
        let features: BTreeSet<(&str, &str)> = ftzr.featurize(&your_string);
        features
    });

    println!();

    let doc = your_doc.lines().map(|x| x.split_ascii_whitespace());
    let doc_ftzr = for_each(for_each(whole()));
    let features: Bag<HashMap<&str, usize>> = doc_ftzr.featurize(doc.clone());
    println!("let your_doc = {:?};", your_doc);
    println!("let doc = your_doc.lines().map(|line| line.split_ascii_whitespace());");
    println!("let doc_ftzr = for_each( for_each( whole() ) );");
    println!(
        "let features: Bag&lt;HashMap&lt;&str, usize&gt;&gt; = doc_ftzr.featurize(doc.clone());"
    );
    println!(">>> {:?}", features);

    println!();

    let features: HashMap<HashedAs<u32>, &str> = doc_ftzr.featurize::<&[u8], _>(doc.clone());
    println!(
        "let features: HashMap&lt;HashedAs&lt;u32&gt;, &str&gt; = doc_ftzr.featurize::<&[u8], _>(doc.clone());"
    );
    println!(">>> {:?}", features);
    end_block(&mut lines);

    println!();

    let doc_gram_ftzr = for_each(for_each(n_slice(a)));
    let features: Vec<&str> = doc_gram_ftzr.featurize(doc);

    println!("let doc_gram_ftzr = for_each(for_each(n_slice({:?})));", a);
    println!("let features: Vec<&str>  = doc_gram_ftzr.featurize(doc);");
    println!(">>> {:?}", features);

    {
        let mut ret: String = "".to_owned();
        for line in lines {
            ret.push_str("\n");
            ret.push_str(line.as_str());
        }
        ret
    }
}
