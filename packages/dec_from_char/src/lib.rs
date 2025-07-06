#![doc = include_str!("../../../Readme.md")]

use dec_from_char_gen::{digit_parse_mappings};

pub trait DecimalExtended where Self: Sized + Copy {
    /// Converts any decimal unicode digit in `Nd` category
    /// into `u32`. Returns `None` if no corresponding digit found.
    fn to_decimal_utf8(&self) -> Option<u32>;
    /// Checks if digit belongs to the `Nd` category
    fn is_decimal_utf8(&self) -> bool {
        self.to_decimal_utf8().is_some()
    }
}

impl DecimalExtended for char {
    fn to_decimal_utf8(&self) -> Option<u32> {
        return digit_parse_mappings!(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncommon_digits() {
        assert_eq!('९'.to_decimal_utf8(), Some(9));
        assert_eq!('०'.to_decimal_utf8(), Some(0));
        assert_eq!('７'.to_decimal_utf8(), Some(7));
        assert_eq!('٣'.to_decimal_utf8(), Some(3));
    }

    #[test]
    fn test_different_formats() {
        let text = "\
        ｗ０-０２３４.３４ｆｗｅ０９８３２４８９２３９ｒ８０)９９ｆｄｓｆ
        𝐰𝟎-𝟎𝟐𝟑𝟒.𝟑𝟒𝐟𝐰𝐞𝟎𝟗𝟖𝟑𝟐𝟒𝟖𝟗𝟐𝟑𝟗𝐫𝟖𝟎)𝟗𝟗𝐟𝐝𝐬𝐟
        𝖜𝟎-𝟎𝟐𝟑𝟒.𝟑𝟒𝖋𝖜𝖊𝟎𝟗𝟖𝟑𝟐𝟒𝟖𝟗𝟐𝟑𝟗𝖗𝟖𝟎)𝟗𝟗𝖋𝖉𝖘𝖋
        𝒘𝟎-𝟎𝟐𝟑𝟒.𝟑𝟒𝒇𝒘𝒆𝟎𝟗𝟖𝟑𝟐𝟒𝟖𝟗𝟐𝟑𝟗𝒓𝟖𝟎)𝟗𝟗𝒇𝒅𝒔𝒇
        𝔀𝟎-𝟎𝟐𝟑𝟒.𝟑𝟒𝓯𝔀𝓮𝟎𝟗𝟖𝟑𝟐𝟒𝟖𝟗𝟐𝟑𝟗𝓻𝟖𝟎)𝟗𝟗𝓯𝓭𝓼𝓯
        𝓌0-0234.34𝒻𝓌𝑒09832489239𝓇80)99𝒻𝒹𝓈𝒻
        𝕨𝟘-𝟘𝟚𝟛𝟜.𝟛𝟜𝕗𝕨𝕖𝟘𝟡𝟠𝟛𝟚𝟜𝟠𝟡𝟚𝟛𝟡𝕣𝟠𝟘)𝟡𝟡𝕗𝕕𝕤𝕗
        𝚠𝟶-𝟶𝟸𝟹𝟺.𝟹𝟺𝚏𝚠𝚎𝟶𝟿𝟾𝟹𝟸𝟺𝟾𝟿𝟸𝟹𝟿𝚛𝟾𝟶)𝟿𝟿𝚏𝚍𝚜𝚏
        𝗐𝟢-𝟢𝟤𝟥𝟦.𝟥𝟦𝖿𝗐𝖾𝟢𝟫𝟪𝟥𝟤𝟦𝟪𝟫𝟤𝟥𝟫𝗋𝟪𝟢)𝟫𝟫𝖿𝖽𝗌𝖿
        𝘄𝟬-𝟬𝟮𝟯𝟰.𝟯𝟰𝗳𝘄𝗲𝟬𝟵𝟴𝟯𝟮𝟰𝟴𝟵𝟮𝟯𝟵𝗿𝟴𝟬)𝟵𝟵𝗳𝗱𝘀𝗳
        𝙬𝟬-𝟬𝟮𝟯𝟰.𝟯𝟰𝙛𝙬𝙚𝟬𝟵𝟴𝟯𝟮𝟰𝟴𝟵𝟮𝟯𝟵𝙧𝟴𝟬)𝟵𝟵𝙛𝙙𝙨𝙛
        𝘸𝟢-𝟢𝟤𝟥𝟦.𝟥𝟦𝘧𝘸𝘦𝟢𝟫𝟪𝟥𝟤𝟦𝟪𝟫𝟤𝟥𝟫𝘳𝟪𝟢)𝟫𝟫𝘧𝘥𝘴𝘧
        🅆0-0234.34🄵🅆🄴09832489239🅁80)99🄵🄳🅂🄵
        🆆0-0234.34🅵🆆🅴09832489239🆁80)99🅵🅳🆂🅵
        🇼​0-0234.34🇫​🇼​🇪​09832489239🇷​80)99🇫​🇩​🇸​🇫
        𝔴0-0234.34𝔣𝔴𝔢09832489239𝔯80)99𝔣𝔡𝔰𝔣
        ẃ0-0234.34f́ẃé09832489239ŕ80)99f́d́śf́
        w̤0-0234.34f̤w̤e̤09832489239r̤80)99f̤d̤s̤f̤
        ẅ0-0234.34f̈ẅë09832489239r̈80)99f̈d̈s̈f̈
        ẅ̤0-0234.34f̤̈ẅ̤ë̤09832489239r̤̈80)99f̤̈d̤̈s̤̈f̤̈
        ̸w0-0234.34̸f̸w̸e09832489239̸r80)99̸f̸d̸s̸f\
        ";

        let mut line_number = 0;
        text.lines()
            .for_each(| line | {
            let parsed = line.chars()
                .filter_map(| c | c.to_decimal_utf8())
                .map(| i | i.to_string())
                .collect::<String>();

            line_number += 1;
            println!("line {}: {}", line_number, line);
            assert_eq!(parsed.as_str(), "0023434098324892398099")
        });

    }
}
