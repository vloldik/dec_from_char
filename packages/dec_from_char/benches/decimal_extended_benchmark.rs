use criterion::{black_box, criterion_group, criterion_main, Criterion};

use core::DecimalExtended;


fn benchmark_decimal_extended_string(c: &mut Criterion) {
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

    c.bench_function("to_decimal_utf8_string", |b| {
        b.iter(|| {
            let _ = black_box(text)
                .chars()
                .filter_map(|c| c.to_decimal_utf8())
                .collect::<Vec<u32>>();
        })
    });
}


fn benchmark_decimal_extended_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_decimal_utf8_char");

    let a = '0';
    let l = &a;

    group.bench_function("ASCII digit with simple decimal", |b| {
        b.iter(|| black_box('7').to_digit(10))
    });

    
    group.bench_function("Full-width digit", |b| {
        b.iter(|| black_box('７').to_decimal_utf8())
    });

    
    group.bench_function("Devanagari digit", |b| {
        b.iter(|| black_box('९').to_decimal_utf8())
    });

    
    group.bench_function("Arabic digit", |b| {
        b.iter(|| black_box('٣').to_decimal_utf8())
    });

    
    group.bench_function("Non-digit", |b| {
        b.iter(|| black_box('a').to_decimal_utf8())
    });

    group.finish();
}

criterion_group!(benches, benchmark_decimal_extended_string, benchmark_decimal_extended_char);
criterion_main!(benches);