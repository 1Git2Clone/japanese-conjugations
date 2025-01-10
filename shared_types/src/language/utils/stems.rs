pub const fn set_a_stem(c: char) -> char {
    match c {
        'す' => 'さ',
        'ぶ' => 'ば',
        'ぐ' => 'が',
        'く' => 'か',
        'む' => 'ま',
        'ぬ' => 'な',
        'る' => 'ら',
        'つ' => 'た',
        'う' => 'あ',
        _ => c,
    }
}

pub const fn set_i_stem(c: char) -> char {
    match c {
        'す' => 'し',
        'ぶ' => 'び',
        'ぐ' => 'ぎ',
        'く' => 'き',
        'む' => 'み',
        'ぬ' => 'に',
        'る' => 'り',
        'つ' => 'ち',
        'う' => 'い',
        _ => c,
    }
}

pub const fn set_e_stem(c: char) -> char {
    match c {
        'す' => 'せ',
        'ぶ' => 'べ',
        'ぐ' => 'げ',
        'く' => 'け',
        'む' => 'め',
        'ぬ' => 'ね',
        'る' => 'れ',
        'つ' => 'て',
        'う' => 'え',
        _ => c,
    }
}

pub const fn set_o_stem(c: char) -> char {
    match c {
        'す' => 'そ',
        'ぶ' => 'ぼ',
        'ぐ' => 'ご',
        'く' => 'こ',
        'む' => 'も',
        'ぬ' => 'の',
        'る' => 'ろ',
        'つ' => 'と',
        'う' => 'お',
        _ => c,
    }
}

pub const fn set_te_form<'a>(c: char) -> &'a str {
    match c {
        'す' => "して",
        'ぶ' => "んで",
        'ぐ' => "いで",
        'く' => "いて",
        'む' => "んで",
        'ぬ' => "んで",
        'る' => "って",
        'つ' => "って",
        'う' => "って",
        _ => "",
    }
}

pub fn map_last_stem_chr<F>(stem: &str, map_last_chr: F) -> char
where
    F: FnOnce(char) -> char,
{
    let Some(last_chr) = stem.chars().last() else {
        return '\0';
    };

    map_last_chr(last_chr)
}

pub fn map_last_stem_chr_to_str<'a, F>(stem: &str, map_last_chr: F) -> &'a str
where
    F: FnOnce(char) -> &'a str,
{
    let Some(last_chr) = stem.chars().last() else {
        return "";
    };

    map_last_chr(last_chr)
}
