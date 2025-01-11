use crate::language::{
    utils::stems::{
        map_last_stem_chr, map_last_stem_chr_to_str, set_a_stem, set_e_stem, set_i_stem,
        set_o_stem, set_te_form,
    },
    verbs::VerbType,
    WordType,
};

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum PlainPoliteness {
    /// 〜る
    Plain,
    /// 〜ます
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum PastPoliteness {
    /// Stem + -n + た
    Plain,
    /// Stem + -i + ました
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum NegativePoliteness {
    /// 〜ない
    Plain,
    /// 〜ません
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum NegativePastPoliteness {
    /// 〜なかった
    Plain,
    /// 〜ませんでした
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum VolitionalPoliteness {
    /// 〜よう
    Plain,
    /// 〜ましょう
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum ImperativePoliteness {
    /// Stem + -e
    Plain,
    /// Stem + -i + なさい
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum EbaPoliteness {
    /// 〜えば
    Plain,
    /// 〜ますれば
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum TaraPoliteness {
    /// 〜たら
    Plain,
    /// 〜ましたら
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum ConditionalType {
    Eba(EbaPoliteness),
    Tara(TaraPoliteness),
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum PotentialPoliteness {
    /// 〜れる
    Plain,
    /// 〜れます
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum PassivePoliteness {
    /// 〜られる
    Plain,
    /// 〜られます
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum CausativePoliteness {
    /// Stem + -a + せる
    Plain,
    /// Stem + -a + せます
    Polite,
}

#[derive(Clone, Copy, PartialEq, Eq, ParseEnum)]
pub enum ConjugationForm {
    /// Language shorthand: `〜`
    Stem,
    Plain(PlainPoliteness),
    Past(PastPoliteness),
    Negative(NegativePoliteness),
    NegativePast(NegativePastPoliteness),
    Volitional(VolitionalPoliteness),
    Imperative(ImperativePoliteness),
    /// 〜たい
    Want,
    Conditional(ConditionalType),
    Potential(PotentialPoliteness),
    /// 〜られる
    Passive(PassivePoliteness),
    /// Stem + -e + よ
    Causative(CausativePoliteness),
    /// Stem + -u + な
    Prohibitive,
    /// Stem + て
    TeForm,
}

pub trait Conjugatable {
    fn conjugate(
        &self,
        stem: &str,
        conjugation_form: ConjugationForm,
        word_type: &WordType,
    ) -> ::syn::LitStr {
        use ConjugationForm as CF;
        use VerbType as VT;
        use WordType as WT;

        if let CF::Stem = conjugation_form {
            return ::syn::LitStr::new(stem, ::proc_macro2::Span::call_site());
        }

        let mut res = stem.to_string();

        match word_type {
            WT::Verb(v) => match v._type {
                VT::Ichidan => match conjugation_form {
                    CF::Stem => unreachable!(),
                    CF::Plain(PlainPoliteness::Plain) => res.push('る'),
                    CF::Plain(PlainPoliteness::Polite) => res.push_str("ます"),
                    CF::Past(PastPoliteness::Plain) => {
                        res.push('た');
                    }
                    CF::Past(PastPoliteness::Polite) => {
                        res.push_str("ました");
                    }
                    CF::Negative(NegativePoliteness::Plain) => {
                        res.push_str("ない");
                    }
                    CF::Negative(NegativePoliteness::Polite) => {
                        res.push_str("ません");
                    }
                    CF::NegativePast(NegativePastPoliteness::Plain) => {
                        res.push_str("なかった");
                    }
                    CF::NegativePast(NegativePastPoliteness::Polite) => {
                        res.push_str("ませんでした");
                    }
                    CF::Volitional(VolitionalPoliteness::Plain) => {
                        res.push_str("よう");
                    }
                    CF::Volitional(VolitionalPoliteness::Polite) => {
                        res.push_str("ましょう");
                    }
                    CF::Imperative(ImperativePoliteness::Plain) => {
                        res.push('ろ');
                    }
                    CF::Imperative(ImperativePoliteness::Polite) => {
                        res.push_str("なさい");
                    }
                    CF::Want => {
                        res.push_str("たい");
                    }
                    CF::Conditional(ConditionalType::Eba(politeness)) => match politeness {
                        EbaPoliteness::Plain => {
                            res.push_str("えば");
                        }
                        EbaPoliteness::Polite => {
                            res.push_str("ますれば");
                        }
                    },
                    CF::Conditional(ConditionalType::Tara(politeness)) => match politeness {
                        TaraPoliteness::Plain => {
                            res.push_str("たら");
                        }
                        TaraPoliteness::Polite => {
                            res.push_str("ましたら");
                        }
                    },
                    CF::Potential(PotentialPoliteness::Plain) => {
                        res.push_str("れる");
                    }
                    CF::Potential(PotentialPoliteness::Polite) => {
                        res.push_str("れます");
                    }
                    CF::Passive(PassivePoliteness::Plain) => {
                        res.push_str("られる");
                    }
                    CF::Passive(PassivePoliteness::Polite) => {
                        res.push_str("られます");
                    }
                    CF::Causative(CausativePoliteness::Plain) => {
                        res.push_str("させる");
                    }
                    CF::Causative(CausativePoliteness::Polite) => {
                        res.push_str("させます");
                    }
                    CF::Prohibitive => {
                        res.push_str("るな");
                    }
                    CF::TeForm => {
                        res.push('て');
                    }
                },
                VT::Godan => match conjugation_form {
                    CF::Stem => unreachable!(),
                    CF::Plain(PlainPoliteness::Plain) => (),
                    CF::Plain(PlainPoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ます");
                    }
                    CF::Past(PastPoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, |_| 'ん'));
                        res.push('だ');
                    }
                    CF::Past(PastPoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ました");
                    }
                    CF::Negative(NegativePoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_a_stem));
                    }
                    CF::Negative(NegativePoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ません");
                    }
                    CF::NegativePast(NegativePastPoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_a_stem));
                        res.push_str("なかった");
                    }
                    CF::NegativePast(NegativePastPoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ませんでした");
                    }
                    CF::Volitional(VolitionalPoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_o_stem));
                        res.push('う')
                    }
                    CF::Volitional(VolitionalPoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ましょう");
                    }
                    CF::Imperative(ImperativePoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_e_stem));
                    }
                    CF::Imperative(ImperativePoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("なさい");
                    }
                    CF::Want => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("たい")
                    }
                    CF::Conditional(ConditionalType::Eba(EbaPoliteness::Plain)) => {
                        res.push(map_last_stem_chr(stem, set_e_stem));
                        res.push('ば')
                    }
                    CF::Conditional(ConditionalType::Eba(EbaPoliteness::Polite)) => {
                        // Doesn't exist.
                    }
                    CF::Conditional(ConditionalType::Tara(TaraPoliteness::Plain)) => {
                        res.push(map_last_stem_chr(stem, |_| 'ん'));
                        res.push_str("たら")
                    }
                    CF::Conditional(ConditionalType::Tara(TaraPoliteness::Polite)) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("ましたら")
                    }
                    CF::Potential(PotentialPoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_e_stem));
                        res.push('る')
                    }
                    CF::Potential(PotentialPoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_e_stem));
                        res.push_str("ます");
                    }
                    CF::Passive(PassivePoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_a_stem));
                        res.push_str("れる");
                    }
                    CF::Passive(PassivePoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_a_stem));
                        res.push_str("れます");
                    }
                    CF::Causative(CausativePoliteness::Plain) => {
                        res.push(map_last_stem_chr(stem, set_a_stem));
                        res.push_str("せる");
                    }
                    CF::Causative(CausativePoliteness::Polite) => {
                        res.push(map_last_stem_chr(stem, set_i_stem));
                        res.push_str("せます");
                    }
                    CF::Prohibitive => {
                        res.push('な');
                    }
                    CF::TeForm => {
                        res.push_str(map_last_stem_chr_to_str(stem, set_te_form));
                    }
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }

        ::syn::LitStr::new(&res, ::proc_macro2::Span::call_site())
    }
}

const INVALID_PATH: &str = "Invalid path";
