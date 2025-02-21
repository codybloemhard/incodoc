use incodoc::{
    parsing::parse,
    output::doc_out,
    reference_doc::REF_DOC,
};

fn main() {
    let res = parse(REF_DOC);
    match res {
        Ok(res) => {
            // println!("{:#?}", res);
            let mut output = String::new();
            doc_out(&res, &mut output);
            println!("{}", output);
        },
        Err(err) => println!("{}", err),
    }
}
