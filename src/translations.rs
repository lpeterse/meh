use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Translations {
    db: Arc<Database>,
    tr: TranslationsTree,
}

impl Translations {
    pub fn new(db: Arc<Database>, fun: Function<Structure<Fix>>) -> Self {
        Self {
            db,
            tr: TranslationsTree::new(fun),
        }
    }
}

impl Iterator for Translations {
    type Item = Text;

    fn next(&mut self) -> Option<Self::Item> {
        self.tr.resolve_all(&self.db)?;
        let mut v = vec![];
        self.tr.collect(&mut v);
        self.tr.inc();
        Some(Text(v))
    }
}

#[derive(Debug)]
struct TranslationsTree {
    fun: Function<Structure<Fix>>,
    idx: usize,
    xxx: Option<Vec<TranslationsNode>>,
}

#[derive(Debug)]
enum TranslationsNode {
    Lit(Literal),
    Fun(TranslationsTree),
}

impl TranslationsTree {
    pub fn new(fun: Function<Structure<Fix>>) -> Self {
        Self {
            fun,
            idx: 0,
            xxx: None,
        }
    }

    fn inc(&mut self) {
        if let Some(xxx) = &mut self.xxx {
            for e in xxx.iter_mut().rev() {
                if let TranslationsNode::Fun(g) = e {
                    g.inc();
                    return;
                }
            }
        }
        self.idx += 1;
        self.xxx = None;
    }

    fn collect(&self, t: &mut Vec<Literal>) {
        if let Some(xxx) = &self.xxx {
            for x in xxx {
                match x {
                    TranslationsNode::Lit(x) => t.push(x.clone()),
                    TranslationsNode::Fun(x) => x.collect(t),
                }
            }
        }
    }

    fn resolve_one(&mut self, db: &Database) -> Option<&mut Vec<TranslationsNode>> {
        if self.xxx.is_none() {
            let (i, es) = db
                .definitions
                .iter()
                .enumerate()
                .skip(self.idx)
                .find_map(|(i, d)| Some((i, d.apply(&self.fun)?)))?;
            let mut xxx = vec![];
            for e in es {
                xxx.push(match e {
                    ExprFix::Lit(x) => TranslationsNode::Lit(x),
                    ExprFix::Fun(x) => TranslationsNode::Fun(TranslationsTree::new(x)),
                });
            }
            self.idx = i;
            self.xxx = Some(xxx);
        }
        self.xxx.as_mut()
    }

    pub fn resolve_all(&mut self, db: &Database) -> Option<()> {
        'outer: loop {
            let xxx = self.resolve_one(db)?;
            let n = xxx.len();
            let mut i = 0;
            let mut k = None;
            'inner: while i < n {
                if let TranslationsNode::Fun(x) = &mut xxx[i] {
                    if x.resolve_all(&db).is_none() {
                        if let Some(k) = k.take() {
                            x.idx = 0;
                            x.xxx = None;
                            if let TranslationsNode::Fun(y) = &mut xxx[k] {
                                y.inc();
                                i = k;
                                continue 'inner;
                            } else {
                                panic!()
                            }
                        } else {
                            self.idx += 1;
                            self.xxx = None;
                            continue 'outer;
                        }
                    } else {
                        k = Some(i);
                    }
                }
                i += 1;
            }
            return Some(());
        }
    }
}
