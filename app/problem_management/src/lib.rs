use proc_macro as pc;
use proc_macro2 as pc2;
use quote::{quote, ToTokens};

#[proc_macro]
pub fn include_problem(item: pc::TokenStream) -> pc::TokenStream {
    let mut p_map = std::collections::HashMap::new();
    let i_str = item.to_string();
    let problems = i_str
        .split(',')
        .map(|s| s.trim())
        .flat_map(|s| {
            let range = s.split("..").collect::<Vec<&str>>();
            match range.len() {
                1 => {
                    vec![range[0].parse().unwrap()]
                }
                2 => {
                    let start = range[0].parse::<u32>().unwrap();
                    let end = range[1].parse::<u32>().unwrap();

                    Vec::from_iter(std::ops::RangeInclusive::new(start, end))
                }
                _ => panic!("Invalid input: range must have only 2 items"),
            }
        })
        .map(|num| {
            let mut problem_string = num.to_string();
            if problem_string.len() == 1 {
                problem_string.insert(0, '0')
            };
            problem_string.insert(0, 'p');
            let ident = pc2::Ident::new(&problem_string, pc2::Span::call_site());
            p_map.insert(num, ident.clone());

            ident
        });
    let mut mod_declare = quote! {};
    for problem in problems {
        mod_declare.extend(quote! {
            pub mod #problem;
        })
    }
    let matchups = p_map.into_iter().map(ProblemTup::new).collect::<Vec<_>>();
    mod_declare.extend(quote! {
        static problem_map: std::collections::HashMap<u32, fn () -> String> = std::collections::HashMap::from([#(#matchups as _),*]);
    });
    mod_declare.into()
}

struct ProblemTup {
    num: u32,
    ident: proc_macro2::Ident
}

impl ProblemTup {
    fn new(tup: (u32, proc_macro2::Ident)) -> Self {
        Self { num: tup.0, ident: tup.1 }
    }
}

impl ToTokens for ProblemTup {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let num = self.num;
        let ident = self.ident.clone();
        *tokens = quote! {
            (#num, #ident::solve)
        }
    }
}