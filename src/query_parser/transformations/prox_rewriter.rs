use crate::query_parser::ast::{Expr, ProximityPart, ProximityTerm, QualifiedField, Term};

pub fn rewrite_proximity_chains(expr: &mut Expr) {
    match expr {
        Expr::Subselect(_, e) => rewrite_proximity_chains(e.as_mut()),
        Expr::Expand(_, e) => rewrite_proximity_chains(e.as_mut()),
        Expr::Not(e) => rewrite_proximity_chains(e.as_mut()),
        Expr::WithList(v) => v.iter_mut().for_each(|e| rewrite_proximity_chains(e)),
        Expr::AndList(v) => v.iter_mut().for_each(|e| rewrite_proximity_chains(e)),
        Expr::OrList(v) => v.iter_mut().for_each(|e| rewrite_proximity_chains(e)),
        Expr::Linked(_, e) => rewrite_proximity_chains(e.as_mut()),

        Expr::Contains(f, t) => rewrite_term(f, t),
        Expr::Eq(f, t) => rewrite_term(f, t),
        Expr::DoesNotContain(f, t) => rewrite_term(f, t),
        Expr::Ne(f, t) => rewrite_term(f, t),

        Expr::Json(_) => {}
        Expr::Gt(_, _) => {}
        Expr::Lt(_, _) => {}
        Expr::Gte(_, _) => {}
        Expr::Lte(_, _) => {}
        Expr::Regex(_, _) => {}
        Expr::MoreLikeThis(_, _) => {}
        Expr::FuzzyLikeThis(_, _) => {}
    }
}

fn rewrite_term(field: &QualifiedField, term: &mut Term) {
    match term {
        Term::PhraseWithWildcard(ref s, parts, b) => {
            // it's a complex phrase with a wildcard, so it needs to be a proximity chain
            match ProximityTerm::make_proximity_chain(field, s, *b) {
                ProximityTerm::ProximityChain(mut v) => parts.append(&mut v),
                other => parts.push(ProximityPart {
                    words: vec![other],
                    distance: None,
                }),
            }
        }
        Term::ProximityChain(v) => {
            for part in v.iter_mut() {
                part.words
                    .iter_mut()
                    .for_each(|prox_term| rewrite_prox_term(field, prox_term));
            }
        }
        _ => {}
    }
}

fn rewrite_prox_term(field: &QualifiedField, prox_term: &mut ProximityTerm) {
    match prox_term {
        ProximityTerm::Phrase(s, b) => {
            *prox_term = ProximityTerm::make_proximity_chain(field, &s, *b)
        }
        ProximityTerm::Prefix(s, b) => {
            *prox_term = ProximityTerm::make_proximity_chain(field, &s, *b)
        }
        ProximityTerm::PhraseWithWildcard(s, _, b) => {
            *prox_term = ProximityTerm::make_proximity_chain(field, &s, *b)
        }
        _ => {}
    }
}
