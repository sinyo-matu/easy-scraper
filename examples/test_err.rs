use std::{borrow::Borrow, ops::Deref};

use kuchiki::traits::*;

fn main() {
    let doc = reqwest::blocking::get(
        "https://pinkhouse-webshop.jp/shopdetail/000000002386/ph/page1/recommend/",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = kuchiki::parse_html().one(doc.trim());
    document.select("#M_price2").unwrap().for_each(|e| {
        println!(
            "{:?}",
            e.as_node()
                .as_element()
                .unwrap()
                .attributes
                .borrow()
                .get("value")
                .unwrap()
        );
    });

    let document = kuchiki::parse_html().one(doc.trim());
    let css2 = ".item_detail_text";
    document.select("#M_price2").unwrap().for_each(|doc| {
        let text = doc
            .as_node()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .select(css2)
            .unwrap()
            .for_each(|doc| {
                println!("{:?}", doc.as_node());
            });
        println!("{:?}", text)
    });
}
