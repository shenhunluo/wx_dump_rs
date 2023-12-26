use crate::SKP_Silk_NLSF_MSVQ_decode::{SKP_Silk_NLSF_CBS, SKP_Silk_NLSF_CB_struct};
use once_cell::sync::Lazy;

pub static SKP_SILK_NLSF_MSVQ_CB0_16_CDF: [u16; 226] = [
    0u16, 1449u16, 2749u16, 4022u16, 5267u16, 6434u16, 7600u16, 8647u16, 9695u16, 10742u16,
    11681u16, 12601u16, 13444u16, 14251u16, 15008u16, 15764u16, 16521u16, 17261u16, 18002u16,
    18710u16, 19419u16, 20128u16, 20837u16, 21531u16, 22225u16, 22919u16, 23598u16, 24277u16,
    24956u16, 25620u16, 26256u16, 26865u16, 27475u16, 28071u16, 28667u16, 29263u16, 29859u16,
    30443u16, 31026u16, 31597u16, 32168u16, 32727u16, 33273u16, 33808u16, 34332u16, 34855u16,
    35379u16, 35902u16, 36415u16, 36927u16, 37439u16, 37941u16, 38442u16, 38932u16, 39423u16,
    39914u16, 40404u16, 40884u16, 41364u16, 41844u16, 42324u16, 42805u16, 43285u16, 43754u16,
    44224u16, 44694u16, 45164u16, 45623u16, 46083u16, 46543u16, 46993u16, 47443u16, 47892u16,
    48333u16, 48773u16, 49213u16, 49653u16, 50084u16, 50515u16, 50946u16, 51377u16, 51798u16,
    52211u16, 52614u16, 53018u16, 53422u16, 53817u16, 54212u16, 54607u16, 55002u16, 55388u16,
    55775u16, 56162u16, 56548u16, 56910u16, 57273u16, 57635u16, 57997u16, 58352u16, 58698u16,
    59038u16, 59370u16, 59702u16, 60014u16, 60325u16, 60630u16, 60934u16, 61239u16, 61537u16,
    61822u16, 62084u16, 62346u16, 62602u16, 62837u16, 63072u16, 63302u16, 63517u16, 63732u16,
    63939u16, 64145u16, 64342u16, 64528u16, 64701u16, 64867u16, 65023u16, 65151u16, 65279u16,
    65407u16, 65535u16, 0u16, 5099u16, 9982u16, 14760u16, 19538u16, 24213u16, 28595u16, 32976u16,
    36994u16, 41012u16, 44944u16, 48791u16, 52557u16, 56009u16, 59388u16, 62694u16, 65535u16, 0u16,
    9955u16, 19697u16, 28825u16, 36842u16, 44686u16, 52198u16, 58939u16, 65535u16, 0u16, 8949u16,
    17335u16, 25720u16, 33926u16, 41957u16, 49987u16, 57845u16, 65535u16, 0u16, 9724u16, 18642u16,
    26998u16, 35355u16, 43532u16, 51534u16, 59365u16, 65535u16, 0u16, 8750u16, 17499u16, 26249u16,
    34448u16, 42471u16, 50494u16, 58178u16, 65535u16, 0u16, 8730u16, 17273u16, 25816u16, 34176u16,
    42536u16, 50203u16, 57869u16, 65535u16, 0u16, 8769u16, 17538u16, 26307u16, 34525u16, 42742u16,
    50784u16, 58319u16, 65535u16, 0u16, 8736u16, 17101u16, 25466u16, 33653u16, 41839u16, 50025u16,
    57864u16, 65535u16, 0u16, 4368u16, 8735u16, 12918u16, 17100u16, 21283u16, 25465u16, 29558u16,
    33651u16, 37744u16, 41836u16, 45929u16, 50022u16, 54027u16, 57947u16, 61782u16, 65535u16,
];

pub static SKP_SILK_NLSF_MSVQ_CB0_16_CDF_START_PTR: Lazy<Vec<&[u16]>> = Lazy::new(|| {
    vec![
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[0..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[129..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[146..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[155..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[164..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[173..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[182..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[191..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[200..],
        &SKP_SILK_NLSF_MSVQ_CB0_16_CDF[209..],
    ]
});

pub static SKP_SILK_NLSF_MSVQ_CB0_16_CDF_MIDDLE_IDX: [i32; 10] = [42, 8, 4, 5, 5, 5, 5, 5, 5, 9];

pub static SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5: [libc::c_short; 216] = [
    176i16, 181i16, 182i16, 183i16, 186i16, 186i16, 191i16, 191i16, 191i16, 196i16, 197i16, 201i16,
    203i16, 206i16, 206i16, 206i16, 207i16, 207i16, 209i16, 209i16, 209i16, 209i16, 210i16, 210i16,
    210i16, 211i16, 211i16, 211i16, 212i16, 214i16, 216i16, 216i16, 217i16, 217i16, 217i16, 217i16,
    218i16, 218i16, 219i16, 219i16, 220i16, 221i16, 222i16, 223i16, 223i16, 223i16, 223i16, 224i16,
    224i16, 224i16, 225i16, 225i16, 226i16, 226i16, 226i16, 226i16, 227i16, 227i16, 227i16, 227i16,
    227i16, 227i16, 228i16, 228i16, 228i16, 228i16, 229i16, 229i16, 229i16, 230i16, 230i16, 230i16,
    231i16, 231i16, 231i16, 231i16, 232i16, 232i16, 232i16, 232i16, 233i16, 234i16, 235i16, 235i16,
    235i16, 236i16, 236i16, 236i16, 236i16, 237i16, 237i16, 237i16, 237i16, 240i16, 240i16, 240i16,
    240i16, 241i16, 242i16, 243i16, 244i16, 244i16, 247i16, 247i16, 248i16, 248i16, 248i16, 249i16,
    251i16, 255i16, 255i16, 256i16, 260i16, 260i16, 261i16, 264i16, 264i16, 266i16, 266i16, 268i16,
    271i16, 274i16, 276i16, 279i16, 288i16, 288i16, 288i16, 288i16, 118i16, 120i16, 121i16, 121i16,
    122i16, 125i16, 125i16, 129i16, 129i16, 130i16, 131i16, 132i16, 136i16, 137i16, 138i16, 145i16,
    87i16, 88i16, 91i16, 97i16, 98i16, 100i16, 105i16, 106i16, 92i16, 95i16, 95i16, 96i16, 97i16,
    97i16, 98i16, 99i16, 88i16, 92i16, 95i16, 95i16, 96i16, 97i16, 98i16, 109i16, 93i16, 93i16,
    93i16, 96i16, 97i16, 97i16, 99i16, 101i16, 93i16, 94i16, 94i16, 95i16, 95i16, 99i16, 99i16,
    99i16, 93i16, 93i16, 93i16, 96i16, 96i16, 97i16, 100i16, 102i16, 93i16, 95i16, 95i16, 96i16,
    96i16, 96i16, 98i16, 99i16, 125i16, 125i16, 127i16, 127i16, 127i16, 127i16, 128i16, 128i16,
    128i16, 128i16, 128i16, 128i16, 129i16, 130i16, 131i16, 132i16,
];

pub static SKP_SILK_NLSF_MSVQ_CB0_16_N_DELTA_MIN_Q15: [i32; 17] = [
    266, 3, 40, 3, 3, 16, 78, 89, 107, 141, 188, 146, 272, 240, 235, 215, 632,
];

pub static SKP_SILK_NLSF_MSVQ_CB0_16_Q15: [i16; 3456] = [
    1170i16, 2278i16, 3658i16, 5374i16, 7666i16, 9113i16, 11298i16, 13304i16, 15371i16, 17549i16,
    19587i16, 21487i16, 23798i16, 26038i16, 28318i16, 30201i16, 1628i16, 2334i16, 4115i16, 6036i16,
    7818i16, 9544i16, 11777i16, 14021i16, 15787i16, 17408i16, 19466i16, 21261i16, 22886i16,
    24565i16, 26714i16, 28059i16, 1724i16, 2670i16, 4056i16, 6532i16, 8357i16, 10119i16, 12093i16,
    14061i16, 16491i16, 18795i16, 20417i16, 22402i16, 24251i16, 26224i16, 28410i16, 29956i16,
    1493i16, 3427i16, 4789i16, 6399i16, 8435i16, 10168i16, 12000i16, 14066i16, 16229i16, 18210i16,
    20040i16, 22098i16, 24153i16, 26095i16, 28183i16, 30121i16, 1119i16, 2089i16, 4295i16, 6245i16,
    8691i16, 10741i16, 12688i16, 15057i16, 17028i16, 18792i16, 20717i16, 22514i16, 24497i16,
    26548i16, 28619i16, 30630i16, 1363i16, 2417i16, 3927i16, 5556i16, 7422i16, 9315i16, 11879i16,
    13767i16, 16143i16, 18520i16, 20458i16, 22578i16, 24539i16, 26436i16, 28318i16, 30318i16,
    1122i16, 2503i16, 5216i16, 7148i16, 9310i16, 11078i16, 13175i16, 14800i16, 16864i16, 18700i16,
    20436i16, 22488i16, 24572i16, 26602i16, 28555i16, 30426i16, 600i16, 1317i16, 2970i16, 5609i16,
    7694i16, 9784i16, 12169i16, 14087i16, 16379i16, 18378i16, 20551i16, 22686i16, 24739i16,
    26697i16, 28646i16, 30355i16, 941i16, 1882i16, 4274i16, 5540i16, 8482i16, 9858i16, 11940i16,
    14287i16, 16091i16, 18501i16, 20326i16, 22612i16, 24711i16, 26638i16, 28814i16, 30430i16,
    635i16, 1699i16, 4376i16, 5948i16, 8097i16, 10115i16, 12274i16, 14178i16, 16111i16, 17813i16,
    19695i16, 21773i16, 23927i16, 25866i16, 28022i16, 30134i16, 1408i16, 2222i16, 3524i16, 5615i16,
    7345i16, 8849i16, 10989i16, 12772i16, 15352i16, 17026i16, 18919i16, 21062i16, 23329i16,
    25215i16, 27209i16, 29023i16, 701i16, 1307i16, 3548i16, 6301i16, 7744i16, 9574i16, 11227i16,
    12978i16, 15170i16, 17565i16, 19775i16, 22097i16, 24230i16, 26335i16, 28377i16, 30231i16,
    1752i16, 2364i16, 4879i16, 6569i16, 7813i16, 9796i16, 11199i16, 14290i16, 15795i16, 18000i16,
    20396i16, 22417i16, 24308i16, 26124i16, 28360i16, 30633i16, 901i16, 1629i16, 3356i16, 4635i16,
    7256i16, 8767i16, 9971i16, 11558i16, 15215i16, 17544i16, 19523i16, 21852i16, 23900i16,
    25978i16, 28133i16, 30184i16, 981i16, 1669i16, 3323i16, 4693i16, 6213i16, 8692i16, 10614i16,
    12956i16, 15211i16, 17711i16, 19856i16, 22122i16, 24344i16, 26592i16, 28723i16, 30481i16,
    1607i16, 2577i16, 4220i16, 5512i16, 8532i16, 10388i16, 11627i16, 13671i16, 15752i16, 17199i16,
    19840i16, 21859i16, 23494i16, 25786i16, 28091i16, 30131i16, 811i16, 1471i16, 3144i16, 5041i16,
    7430i16, 9389i16, 11174i16, 13255i16, 15157i16, 16741i16, 19583i16, 22167i16, 24115i16,
    26142i16, 28383i16, 30395i16, 1543i16, 2144i16, 3629i16, 6347i16, 7333i16, 9339i16, 10710i16,
    13596i16, 15099i16, 17340i16, 20102i16, 21886i16, 23732i16, 25637i16, 27818i16, 29917i16,
    492i16, 1185i16, 2940i16, 5488i16, 7095i16, 8751i16, 11596i16, 13579i16, 16045i16, 18015i16,
    20178i16, 22127i16, 24265i16, 26406i16, 28484i16, 30357i16, 1547i16, 2282i16, 3693i16, 6341i16,
    7758i16, 9607i16, 11848i16, 13236i16, 16564i16, 18069i16, 19759i16, 21404i16, 24110i16,
    26606i16, 28786i16, 30655i16, 685i16, 1338i16, 3409i16, 5262i16, 6950i16, 9222i16, 11414i16,
    14523i16, 16337i16, 17893i16, 19436i16, 21298i16, 23293i16, 25181i16, 27973i16, 30520i16,
    887i16, 1581i16, 3057i16, 4318i16, 7192i16, 8617i16, 10047i16, 13106i16, 16265i16, 17893i16,
    20233i16, 22350i16, 24379i16, 26384i16, 28314i16, 30189i16, 2285i16, 3745i16, 5662i16, 7576i16,
    9323i16, 11320i16, 13239i16, 15191i16, 17175i16, 19225i16, 21108i16, 22972i16, 24821i16,
    26655i16, 28561i16, 30460i16, 1496i16, 2108i16, 3448i16, 6898i16, 8328i16, 9656i16, 11252i16,
    12823i16, 14979i16, 16482i16, 18180i16, 20085i16, 22962i16, 25160i16, 27705i16, 29629i16,
    575i16, 1261i16, 3861i16, 6627i16, 8294i16, 10809i16, 12705i16, 14768i16, 17076i16, 19047i16,
    20978i16, 23055i16, 24972i16, 26703i16, 28720i16, 30345i16, 1682i16, 2213i16, 3882i16, 6238i16,
    7208i16, 9646i16, 10877i16, 13431i16, 14805i16, 16213i16, 17941i16, 20873i16, 23550i16,
    25765i16, 27756i16, 29461i16, 888i16, 1616i16, 3924i16, 5195i16, 7206i16, 8647i16, 9842i16,
    11473i16, 16067i16, 18221i16, 20343i16, 22774i16, 24503i16, 26412i16, 28054i16, 29731i16,
    805i16, 1454i16, 2683i16, 4472i16, 7936i16, 9360i16, 11398i16, 14345i16, 16205i16, 17832i16,
    19453i16, 21646i16, 23899i16, 25928i16, 28387i16, 30463i16, 1640i16, 2383i16, 3484i16, 5082i16,
    6032i16, 8606i16, 11640i16, 12966i16, 15842i16, 17368i16, 19346i16, 21182i16, 23638i16,
    25889i16, 28368i16, 30299i16, 1632i16, 2204i16, 4510i16, 7580i16, 8718i16, 10512i16, 11962i16,
    14096i16, 15640i16, 17194i16, 19143i16, 22247i16, 24563i16, 26561i16, 28604i16, 30509i16,
    2043i16, 2612i16, 3985i16, 6851i16, 8038i16, 9514i16, 10979i16, 12789i16, 15426i16, 16728i16,
    18899i16, 20277i16, 22902i16, 26209i16, 28711i16, 30618i16, 2224i16, 2798i16, 4465i16, 5320i16,
    7108i16, 9436i16, 10986i16, 13222i16, 14599i16, 18317i16, 20141i16, 21843i16, 23601i16,
    25700i16, 28184i16, 30582i16, 835i16, 1541i16, 4083i16, 5769i16, 7386i16, 9399i16, 10971i16,
    12456i16, 15021i16, 18642i16, 20843i16, 23100i16, 25292i16, 26966i16, 28952i16, 30422i16,
    1795i16, 2343i16, 4809i16, 5896i16, 7178i16, 8545i16, 10223i16, 13370i16, 14606i16, 16469i16,
    18273i16, 20736i16, 23645i16, 26257i16, 28224i16, 30390i16, 1734i16, 2254i16, 4031i16, 5188i16,
    6506i16, 7872i16, 9651i16, 13025i16, 14419i16, 17305i16, 19495i16, 22190i16, 24403i16,
    26302i16, 28195i16, 30177i16, 1841i16, 2349i16, 3968i16, 4764i16, 6376i16, 9825i16, 11048i16,
    13345i16, 14682i16, 16252i16, 18183i16, 21363i16, 23918i16, 26156i16, 28031i16, 29935i16,
    1432i16, 2047i16, 5631i16, 6927i16, 8198i16, 9675i16, 11358i16, 13506i16, 14802i16, 16419i16,
    18339i16, 22019i16, 24124i16, 26177i16, 28130i16, 30586i16, 1730i16, 2320i16, 3744i16, 4808i16,
    6007i16, 9666i16, 10997i16, 13622i16, 15234i16, 17495i16, 20088i16, 22002i16, 23603i16,
    25400i16, 27379i16, 29254i16, 1267i16, 1915i16, 5483i16, 6812i16, 8229i16, 9919i16, 11589i16,
    13337i16, 14747i16, 17965i16, 20552i16, 22167i16, 24519i16, 26819i16, 28883i16, 30642i16,
    1526i16, 2229i16, 4240i16, 7388i16, 8953i16, 10450i16, 11899i16, 13718i16, 16861i16, 18323i16,
    20379i16, 22672i16, 24797i16, 26906i16, 28906i16, 30622i16, 2175i16, 2791i16, 4104i16, 6875i16,
    8612i16, 9798i16, 12152i16, 13536i16, 15623i16, 17682i16, 19213i16, 21060i16, 24382i16,
    26760i16, 28633i16, 30248i16, 454i16, 1231i16, 4339i16, 5738i16, 7550i16, 9006i16, 10320i16,
    13525i16, 16005i16, 17849i16, 20071i16, 21992i16, 23949i16, 26043i16, 28245i16, 30175i16,
    2250i16, 2791i16, 4230i16, 5283i16, 6762i16, 10607i16, 11879i16, 13821i16, 15797i16, 17264i16,
    20029i16, 22266i16, 24588i16, 26437i16, 28244i16, 30419i16, 1696i16, 2216i16, 4308i16, 8385i16,
    9766i16, 11030i16, 12556i16, 14099i16, 16322i16, 17640i16, 19166i16, 20590i16, 23967i16,
    26858i16, 28798i16, 30562i16, 2452i16, 3236i16, 4369i16, 6118i16, 7156i16, 9003i16, 11509i16,
    12796i16, 15749i16, 17291i16, 19491i16, 22241i16, 24530i16, 26474i16, 28273i16, 30073i16,
    1811i16, 2541i16, 3555i16, 5480i16, 9123i16, 10527i16, 11894i16, 13659i16, 15262i16, 16899i16,
    19366i16, 21069i16, 22694i16, 24314i16, 27256i16, 29983i16, 1553i16, 2246i16, 4559i16, 5500i16,
    6754i16, 7874i16, 11739i16, 13571i16, 15188i16, 17879i16, 20281i16, 22510i16, 24614i16,
    26649i16, 28786i16, 30755i16, 1982i16, 2768i16, 3834i16, 5964i16, 8732i16, 9908i16, 11797i16,
    14813i16, 16311i16, 17946i16, 21097i16, 22851i16, 24456i16, 26304i16, 28166i16, 29755i16,
    1824i16, 2529i16, 3817i16, 5449i16, 6854i16, 8714i16, 10381i16, 12286i16, 14194i16, 15774i16,
    19524i16, 21374i16, 23695i16, 26069i16, 28096i16, 30212i16, 2212i16, 2854i16, 3947i16, 5898i16,
    9930i16, 11556i16, 12854i16, 14788i16, 16328i16, 17700i16, 20321i16, 22098i16, 23672i16,
    25291i16, 26976i16, 28586i16, 2023i16, 2599i16, 4024i16, 4916i16, 6613i16, 11149i16, 12457i16,
    14626i16, 16320i16, 17822i16, 19673i16, 21172i16, 23115i16, 26051i16, 28825i16, 30758i16,
    1628i16, 2206i16, 3467i16, 4364i16, 8679i16, 10173i16, 11864i16, 13679i16, 14998i16, 16938i16,
    19207i16, 21364i16, 23850i16, 26115i16, 28124i16, 30273i16, 2014i16, 2603i16, 4114i16, 7254i16,
    8516i16, 10043i16, 11822i16, 13503i16, 16329i16, 17826i16, 19697i16, 21280i16, 23151i16,
    24661i16, 26807i16, 30161i16, 2376i16, 2980i16, 4422i16, 5770i16, 7016i16, 9723i16, 11125i16,
    13516i16, 15485i16, 16985i16, 19160i16, 20587i16, 24401i16, 27180i16, 29046i16, 30647i16,
    2454i16, 3502i16, 4624i16, 6019i16, 7632i16, 8849i16, 10792i16, 13964i16, 15523i16, 17085i16,
    19611i16, 21238i16, 22856i16, 25108i16, 28106i16, 29890i16, 1573i16, 2274i16, 3308i16, 5999i16,
    8977i16, 10104i16, 12457i16, 14258i16, 15749i16, 18180i16, 19974i16, 21253i16, 23045i16,
    25058i16, 27741i16, 30315i16, 1943i16, 2730i16, 4140i16, 6160i16, 7491i16, 8986i16, 11309i16,
    12775i16, 14820i16, 16558i16, 17909i16, 19757i16, 21512i16, 23605i16, 27274i16, 29527i16,
    2021i16, 2582i16, 4494i16, 5835i16, 6993i16, 8245i16, 9827i16, 14733i16, 16462i16, 17894i16,
    19647i16, 21083i16, 23764i16, 26667i16, 29072i16, 30990i16, 1052i16, 1775i16, 3218i16, 4378i16,
    7666i16, 9403i16, 11248i16, 13327i16, 14972i16, 17962i16, 20758i16, 22354i16, 25071i16,
    27209i16, 29001i16, 30609i16, 2218i16, 2866i16, 4223i16, 5352i16, 6581i16, 9980i16, 11587i16,
    13121i16, 15193i16, 16583i16, 18386i16, 20080i16, 22013i16, 25317i16, 28127i16, 29880i16,
    2146i16, 2840i16, 4397i16, 5840i16, 7449i16, 8721i16, 10512i16, 11936i16, 13595i16, 17253i16,
    19310i16, 20891i16, 23417i16, 25627i16, 27749i16, 30231i16, 1972i16, 2619i16, 3756i16, 6367i16,
    7641i16, 8814i16, 12286i16, 13768i16, 15309i16, 18036i16, 19557i16, 20904i16, 22582i16,
    24876i16, 27800i16, 30440i16, 2005i16, 2577i16, 4272i16, 7373i16, 8558i16, 10223i16, 11770i16,
    13402i16, 16502i16, 18000i16, 19645i16, 21104i16, 22990i16, 26806i16, 29505i16, 30942i16,
    1153i16, 1822i16, 3724i16, 5443i16, 6990i16, 8702i16, 10289i16, 11899i16, 13856i16, 15315i16,
    17601i16, 21064i16, 23692i16, 26083i16, 28586i16, 30639i16, 1304i16, 1869i16, 3318i16, 7195i16,
    9613i16, 10733i16, 12393i16, 13728i16, 15822i16, 17474i16, 18882i16, 20692i16, 23114i16,
    25540i16, 27684i16, 29244i16, 2093i16, 2691i16, 4018i16, 6658i16, 7947i16, 9147i16, 10497i16,
    11881i16, 15888i16, 17821i16, 19333i16, 21233i16, 23371i16, 25234i16, 27553i16, 29998i16,
    575i16, 1331i16, 5304i16, 6910i16, 8425i16, 10086i16, 11577i16, 13498i16, 16444i16, 18527i16,
    20565i16, 22847i16, 24914i16, 26692i16, 28759i16, 30157i16, 1435i16, 2024i16, 3283i16, 4156i16,
    7611i16, 10592i16, 12049i16, 13927i16, 15459i16, 18413i16, 20495i16, 22270i16, 24222i16,
    26093i16, 28065i16, 30099i16, 1632i16, 2168i16, 5540i16, 7478i16, 8630i16, 10391i16, 11644i16,
    14321i16, 15741i16, 17357i16, 18756i16, 20434i16, 22799i16, 26060i16, 28542i16, 30696i16,
    1407i16, 2245i16, 3405i16, 5639i16, 9419i16, 10685i16, 12104i16, 13495i16, 15535i16, 18357i16,
    19996i16, 21689i16, 24351i16, 26550i16, 28853i16, 30564i16, 1675i16, 2226i16, 4005i16, 8223i16,
    9975i16, 11155i16, 12822i16, 14316i16, 16504i16, 18137i16, 19574i16, 21050i16, 22759i16,
    24912i16, 28296i16, 30634i16, 1080i16, 1614i16, 3622i16, 7565i16, 8748i16, 10303i16, 11713i16,
    13848i16, 15633i16, 17434i16, 19761i16, 21825i16, 23571i16, 25393i16, 27406i16, 29063i16,
    1693i16, 2229i16, 3456i16, 4354i16, 5670i16, 10890i16, 12563i16, 14167i16, 15879i16, 17377i16,
    19817i16, 21971i16, 24094i16, 26131i16, 28298i16, 30099i16, 2042i16, 2959i16, 4195i16, 5740i16,
    7106i16, 8267i16, 11126i16, 14973i16, 16914i16, 18295i16, 20532i16, 21982i16, 23711i16,
    25769i16, 27609i16, 29351i16, 984i16, 1612i16, 3808i16, 5265i16, 6885i16, 8411i16, 9547i16,
    10889i16, 12522i16, 16520i16, 19549i16, 21639i16, 23746i16, 26058i16, 28310i16, 30374i16,
    2036i16, 2538i16, 4166i16, 7761i16, 9146i16, 10412i16, 12144i16, 13609i16, 15588i16, 17169i16,
    18559i16, 20113i16, 21820i16, 24313i16, 28029i16, 30612i16, 1871i16, 2355i16, 4061i16, 5143i16,
    7464i16, 10129i16, 11941i16, 15001i16, 16680i16, 18354i16, 19957i16, 22279i16, 24861i16,
    26872i16, 28988i16, 30615i16, 2566i16, 3161i16, 4643i16, 6227i16, 7406i16, 9970i16, 11618i16,
    13416i16, 15889i16, 17364i16, 19121i16, 20817i16, 22592i16, 24720i16, 28733i16, 31082i16,
    1700i16, 2327i16, 4828i16, 5939i16, 7567i16, 9154i16, 11087i16, 12771i16, 14209i16, 16121i16,
    20222i16, 22671i16, 24648i16, 26656i16, 28696i16, 30745i16, 3169i16, 3873i16, 5046i16, 6868i16,
    8184i16, 9480i16, 12335i16, 14068i16, 15774i16, 17971i16, 20231i16, 21711i16, 23520i16,
    25245i16, 27026i16, 28730i16, 1564i16, 2391i16, 4229i16, 6730i16, 8905i16, 10459i16, 13026i16,
    15033i16, 17265i16, 19809i16, 21849i16, 23741i16, 25490i16, 27312i16, 29061i16, 30527i16,
    2864i16, 3559i16, 4719i16, 6441i16, 9592i16, 11055i16, 12763i16, 14784i16, 16428i16, 18164i16,
    20486i16, 22262i16, 24183i16, 26263i16, 28383i16, 30224i16, 2673i16, 3449i16, 4581i16, 5983i16,
    6863i16, 8311i16, 12464i16, 13911i16, 15738i16, 17791i16, 19416i16, 21182i16, 24025i16,
    26561i16, 28723i16, 30440i16, 2419i16, 3049i16, 4274i16, 6384i16, 8564i16, 9661i16, 11288i16,
    12676i16, 14447i16, 17578i16, 19816i16, 21231i16, 23099i16, 25270i16, 26899i16, 28926i16,
    1278i16, 2001i16, 3000i16, 5353i16, 9995i16, 11777i16, 13018i16, 14570i16, 16050i16, 17762i16,
    19982i16, 21617i16, 23371i16, 25083i16, 27656i16, 30172i16, 932i16, 1624i16, 2798i16, 4570i16,
    8592i16, 9988i16, 11552i16, 13050i16, 16921i16, 18677i16, 20415i16, 22810i16, 24817i16,
    26819i16, 28804i16, 30385i16, 2324i16, 2973i16, 4156i16, 5702i16, 6919i16, 8806i16, 10259i16,
    12503i16, 15015i16, 16567i16, 19418i16, 21375i16, 22943i16, 24550i16, 27024i16, 29849i16,
    1564i16, 2373i16, 3455i16, 4907i16, 5975i16, 7436i16, 11786i16, 14505i16, 16107i16, 18148i16,
    20019i16, 21653i16, 23740i16, 25814i16, 28578i16, 30372i16, 3025i16, 3729i16, 4866i16, 6520i16,
    9487i16, 10943i16, 12358i16, 14258i16, 16174i16, 17501i16, 19476i16, 21408i16, 23227i16,
    24906i16, 27347i16, 29407i16, 1270i16, 1965i16, 6802i16, 7995i16, 9204i16, 10828i16, 12507i16,
    14230i16, 15759i16, 17860i16, 20369i16, 22502i16, 24633i16, 26514i16, 28535i16, 30525i16,
    2210i16, 2749i16, 4266i16, 7487i16, 9878i16, 11018i16, 12823i16, 14431i16, 16247i16, 18626i16,
    20450i16, 22054i16, 23739i16, 25291i16, 27074i16, 29169i16, 1275i16, 1926i16, 4330i16, 6573i16,
    8441i16, 10920i16, 13260i16, 15008i16, 16927i16, 18573i16, 20644i16, 22217i16, 23983i16,
    25474i16, 27372i16, 28645i16, 3015i16, 3670i16, 5086i16, 6372i16, 7888i16, 9309i16, 10966i16,
    12642i16, 14495i16, 16172i16, 18080i16, 19972i16, 22454i16, 24899i16, 27362i16, 29975i16,
    2882i16, 3733i16, 5113i16, 6482i16, 8125i16, 9685i16, 11598i16, 13288i16, 15405i16, 17192i16,
    20178i16, 22426i16, 24801i16, 27014i16, 29212i16, 30811i16, 2300i16, 2968i16, 4101i16, 5442i16,
    6327i16, 7910i16, 12455i16, 13862i16, 15747i16, 17505i16, 19053i16, 20679i16, 22615i16,
    24658i16, 27499i16, 30065i16, 2257i16, 2940i16, 4430i16, 5991i16, 7042i16, 8364i16, 9414i16,
    11224i16, 15723i16, 17420i16, 19253i16, 21469i16, 23915i16, 26053i16, 28430i16, 30384i16,
    1227i16, 2045i16, 3818i16, 5011i16, 6990i16, 9231i16, 11024i16, 13011i16, 17341i16, 19017i16,
    20583i16, 22799i16, 25195i16, 26876i16, 29351i16, 30805i16, 1354i16, 1924i16, 3789i16, 8077i16,
    10453i16, 11639i16, 13352i16, 14817i16, 16743i16, 18189i16, 20095i16, 22014i16, 24593i16,
    26677i16, 28647i16, 30256i16, 3142i16, 4049i16, 6197i16, 7417i16, 8753i16, 10156i16, 11533i16,
    13181i16, 15947i16, 17655i16, 19606i16, 21402i16, 23487i16, 25659i16, 28123i16, 30304i16,
    1317i16, 2263i16, 4725i16, 7611i16, 9667i16, 11634i16, 14143i16, 16258i16, 18724i16, 20698i16,
    22379i16, 24007i16, 25775i16, 27251i16, 28930i16, 30593i16, 1570i16, 2323i16, 3818i16, 6215i16,
    9893i16, 11556i16, 13070i16, 14631i16, 16152i16, 18290i16, 21386i16, 23346i16, 25114i16,
    26923i16, 28712i16, 30168i16, 2297i16, 3905i16, 6287i16, 8558i16, 10668i16, 12766i16, 15019i16,
    17102i16, 19036i16, 20677i16, 22341i16, 23871i16, 25478i16, 27085i16, 28851i16, 30520i16,
    1915i16, 2507i16, 4033i16, 5749i16, 7059i16, 8871i16, 10659i16, 12198i16, 13937i16, 15383i16,
    16869i16, 18707i16, 23175i16, 25818i16, 28514i16, 30501i16, 2404i16, 2918i16, 5190i16, 6252i16,
    7426i16, 9887i16, 12387i16, 14795i16, 16754i16, 18368i16, 20338i16, 22003i16, 24236i16,
    26456i16, 28490i16, 30397i16, 1621i16, 2227i16, 3479i16, 5085i16, 9425i16, 12892i16, 14246i16,
    15652i16, 17205i16, 18674i16, 20446i16, 22209i16, 23778i16, 25867i16, 27931i16, 30093i16,
    1869i16, 2390i16, 4105i16, 7021i16, 11221i16, 12775i16, 14059i16, 15590i16, 17024i16, 18608i16,
    20595i16, 22075i16, 23649i16, 25154i16, 26914i16, 28671i16, 2551i16, 3252i16, 4688i16, 6562i16,
    7869i16, 9125i16, 10475i16, 11800i16, 15402i16, 18780i16, 20992i16, 22555i16, 24289i16,
    25968i16, 27465i16, 29232i16, 2705i16, 3493i16, 4735i16, 6360i16, 7905i16, 9352i16, 11538i16,
    13430i16, 15239i16, 16919i16, 18619i16, 20094i16, 21800i16, 23342i16, 25200i16, 29257i16,
    2166i16, 2791i16, 4011i16, 5081i16, 5896i16, 9038i16, 13407i16, 14703i16, 16543i16, 18189i16,
    19896i16, 21857i16, 24872i16, 26971i16, 28955i16, 30514i16, 1865i16, 3021i16, 4696i16, 6534i16,
    8343i16, 9914i16, 12789i16, 14103i16, 16533i16, 17729i16, 21340i16, 22439i16, 24873i16,
    26330i16, 28428i16, 30154i16, 3369i16, 4345i16, 6573i16, 8763i16, 10309i16, 11713i16, 13367i16,
    14784i16, 16483i16, 18145i16, 19839i16, 21247i16, 23292i16, 25477i16, 27555i16, 29447i16,
    1265i16, 2184i16, 5443i16, 7893i16, 10591i16, 13139i16, 15105i16, 16639i16, 18402i16, 19826i16,
    21419i16, 22995i16, 24719i16, 26437i16, 28363i16, 30125i16, 1584i16, 2004i16, 3535i16, 4450i16,
    8662i16, 10764i16, 12832i16, 14978i16, 16972i16, 18794i16, 20932i16, 22547i16, 24636i16,
    26521i16, 28701i16, 30567i16, 3419i16, 4528i16, 6602i16, 7890i16, 9508i16, 10875i16, 12771i16,
    14357i16, 16051i16, 18330i16, 20630i16, 22490i16, 25070i16, 26936i16, 28946i16, 30542i16,
    1726i16, 2252i16, 4597i16, 6950i16, 8379i16, 9823i16, 11363i16, 12794i16, 14306i16, 15476i16,
    16798i16, 18018i16, 21671i16, 25550i16, 28148i16, 30367i16, 3385i16, 3870i16, 5307i16, 6388i16,
    7141i16, 8684i16, 12695i16, 14939i16, 16480i16, 18277i16, 20537i16, 22048i16, 23947i16,
    25965i16, 28214i16, 29956i16, 2771i16, 3306i16, 4450i16, 5560i16, 6453i16, 9493i16, 13548i16,
    14754i16, 16743i16, 18447i16, 20028i16, 21736i16, 23746i16, 25353i16, 27141i16, 29066i16,
    3028i16, 3900i16, 6617i16, 7893i16, 9211i16, 10480i16, 12047i16, 13583i16, 15182i16, 16662i16,
    18502i16, 20092i16, 22190i16, 24358i16, 26302i16, 28957i16, 2000i16, 2550i16, 4067i16, 6837i16,
    9628i16, 11002i16, 12594i16, 14098i16, 15589i16, 17195i16, 18679i16, 20099i16, 21530i16,
    23085i16, 24641i16, 29022i16, 2844i16, 3302i16, 5103i16, 6107i16, 6911i16, 8598i16, 12416i16,
    14054i16, 16026i16, 18567i16, 20672i16, 22270i16, 23952i16, 25771i16, 27658i16, 30026i16,
    4043i16, 5150i16, 7268i16, 9056i16, 10916i16, 12638i16, 14543i16, 16184i16, 17948i16, 19691i16,
    21357i16, 22981i16, 24825i16, 26591i16, 28479i16, 30233i16, 2109i16, 2625i16, 4320i16, 5525i16,
    7454i16, 10220i16, 12980i16, 14698i16, 17627i16, 19263i16, 20485i16, 22381i16, 24279i16,
    25777i16, 27847i16, 30458i16, 1550i16, 2667i16, 6473i16, 9496i16, 10985i16, 12352i16, 13795i16,
    15233i16, 17099i16, 18642i16, 20461i16, 22116i16, 24197i16, 26291i16, 28403i16, 30132i16,
    2411i16, 3084i16, 4145i16, 5394i16, 6367i16, 8154i16, 13125i16, 16049i16, 17561i16, 19125i16,
    21258i16, 22762i16, 24459i16, 26317i16, 28255i16, 29702i16, 4159i16, 4516i16, 5956i16, 7635i16,
    8254i16, 8980i16, 11208i16, 14133i16, 16210i16, 17875i16, 20196i16, 21864i16, 23840i16,
    25747i16, 28058i16, 30012i16, 2026i16, 2431i16, 2845i16, 3618i16, 7950i16, 9802i16, 12721i16,
    14460i16, 16576i16, 18984i16, 21376i16, 23319i16, 24961i16, 26718i16, 28971i16, 30640i16,
    3429i16, 3833i16, 4472i16, 4912i16, 7723i16, 10386i16, 12981i16, 15322i16, 16699i16, 18807i16,
    20778i16, 22551i16, 24627i16, 26494i16, 28334i16, 30482i16, 4740i16, 5169i16, 5796i16, 6485i16,
    6998i16, 8830i16, 11777i16, 14414i16, 16831i16, 18413i16, 20789i16, 22369i16, 24236i16,
    25835i16, 27807i16, 30021i16, 150i16, 168i16, -17i16, -107i16, -142i16, -229i16, -320i16,
    -406i16, -503i16, -620i16, -867i16, -935i16, -902i16, -680i16, -398i16, -114i16, -398i16,
    -355i16, 49i16, 255i16, 114i16, 260i16, 399i16, 264i16, 317i16, 431i16, 514i16, 531i16, 435i16,
    356i16, 238i16, 106i16, -43i16, -36i16, -169i16, -224i16, -391i16, -633i16, -776i16, -970i16,
    -844i16, -455i16, -181i16, -12i16, 85i16, 85i16, 164i16, 195i16, 122i16, 85i16, -158i16,
    -640i16, -903i16, 9i16, 7i16, -124i16, 149i16, 32i16, 220i16, 369i16, 242i16, 115i16, 79i16,
    84i16, -146i16, -216i16, -70i16, 1024i16, 751i16, 574i16, 440i16, 377i16, 352i16, 203i16,
    30i16, 16i16, -3i16, 81i16, 161i16, 100i16, -148i16, -176i16, 933i16, 750i16, 404i16, 171i16,
    -2i16, -146i16, -411i16, -442i16, -541i16, -552i16, -442i16, -269i16, -240i16, -52i16, 603i16,
    635i16, 405i16, 178i16, 215i16, 19i16, -153i16, -167i16, -290i16, -219i16, 151i16, 271i16,
    151i16, 119i16, 303i16, 266i16, 100i16, 69i16, -293i16, -657i16, 939i16, 659i16, 442i16,
    351i16, 132i16, 98i16, -16i16, -1i16, -135i16, -200i16, -223i16, -89i16, 167i16, 154i16,
    172i16, 237i16, -45i16, -183i16, -228i16, -486i16, 263i16, 608i16, 158i16, -125i16, -390i16,
    -227i16, -118i16, 43i16, -457i16, -392i16, -769i16, -840i16, 20i16, -117i16, -194i16, -189i16,
    -173i16, -173i16, -33i16, 32i16, 174i16, 144i16, 115i16, 167i16, 57i16, 44i16, 14i16, 147i16,
    96i16, -54i16, -142i16, -129i16, -254i16, -331i16, 304i16, 310i16, -52i16, -419i16, -846i16,
    -1060i16, -88i16, -123i16, -202i16, -343i16, -554i16, -961i16, -951i16, 327i16, 159i16, 81i16,
    255i16, 227i16, 120i16, 203i16, 256i16, 192i16, 164i16, 224i16, 290i16, 195i16, 216i16, 209i16,
    128i16, 832i16, 1028i16, 889i16, 698i16, 504i16, 408i16, 355i16, 218i16, 32i16, -115i16,
    -84i16, -276i16, -100i16, -312i16, -484i16, 899i16, 682i16, 465i16, 456i16, 241i16, -12i16,
    -275i16, -425i16, -461i16, -367i16, -33i16, -28i16, -102i16, -194i16, -527i16, 863i16, 906i16,
    463i16, 245i16, 13i16, -212i16, -305i16, -105i16, 163i16, 279i16, 176i16, 93i16, 67i16, 115i16,
    192i16, 61i16, -50i16, -132i16, -175i16, -224i16, -271i16, -629i16, -252i16, 1158i16, 972i16,
    638i16, 280i16, 300i16, 326i16, 143i16, -152i16, -214i16, -287i16, 53i16, -42i16, -236i16,
    -352i16, -423i16, -248i16, -129i16, -163i16, -178i16, -119i16, 85i16, 57i16, 514i16, 382i16,
    374i16, 402i16, 424i16, 423i16, 271i16, 197i16, 97i16, 40i16, 39i16, -97i16, -191i16, -164i16,
    -230i16, -256i16, -410i16, 396i16, 327i16, 127i16, 10i16, -119i16, -167i16, -291i16, -274i16,
    -141i16, -99i16, -226i16, -218i16, -139i16, -224i16, -209i16, -268i16, -442i16, -413i16,
    222i16, 58i16, 521i16, 344i16, 258i16, 76i16, -42i16, -142i16, -165i16, -123i16, -92i16, 47i16,
    8i16, -3i16, -191i16, -11i16, -164i16, -167i16, -351i16, -740i16, 311i16, 538i16, 291i16,
    184i16, 29i16, -105i16, 9i16, -30i16, -54i16, -17i16, -77i16, -271i16, -412i16, -622i16,
    -648i16, 476i16, 186i16, -66i16, -197i16, -73i16, -94i16, -15i16, 47i16, 28i16, 112i16, -58i16,
    -33i16, 65i16, 19i16, 84i16, 86i16, 276i16, 114i16, 472i16, 786i16, 799i16, 625i16, 415i16,
    178i16, -35i16, -26i16, 5i16, 9i16, 83i16, 39i16, 37i16, 39i16, -184i16, -374i16, -265i16,
    -362i16, -501i16, 337i16, 716i16, 478i16, -60i16, -125i16, -163i16, 362i16, 17i16, -122i16,
    -233i16, 279i16, 138i16, 157i16, 318i16, 193i16, 189i16, 209i16, 266i16, 252i16, -46i16,
    -56i16, -277i16, -429i16, 464i16, 386i16, 142i16, 44i16, -43i16, 66i16, 264i16, 182i16, 47i16,
    14i16, -26i16, -79i16, 49i16, 15i16, -128i16, -203i16, -400i16, -478i16, 325i16, 27i16, 234i16,
    411i16, 205i16, 129i16, 12i16, 58i16, 123i16, 57i16, 171i16, 137i16, 96i16, 128i16, -32i16,
    134i16, -12i16, 57i16, 119i16, 26i16, -22i16, -165i16, -500i16, -701i16, -528i16, -116i16,
    64i16, -8i16, 97i16, -9i16, -162i16, -66i16, -156i16, -194i16, -303i16, -546i16, -341i16,
    546i16, 358i16, 95i16, 45i16, 76i16, 270i16, 403i16, 205i16, 100i16, 123i16, 50i16, -53i16,
    -144i16, -110i16, -13i16, 32i16, -228i16, -130i16, 353i16, 296i16, 56i16, -372i16, -253i16,
    365i16, 73i16, 10i16, -34i16, -139i16, -191i16, -96i16, 5i16, 44i16, -85i16, -179i16, -129i16,
    -192i16, -246i16, -85i16, -110i16, -155i16, -44i16, -27i16, 145i16, 138i16, 79i16, 32i16,
    -148i16, -577i16, -634i16, 191i16, 94i16, -9i16, -35i16, -77i16, -84i16, -56i16, -171i16,
    -298i16, -271i16, -243i16, -156i16, -328i16, -235i16, -76i16, -128i16, -121i16, 129i16, 13i16,
    -22i16, 32i16, 45i16, -248i16, -65i16, 193i16, -81i16, 299i16, 57i16, -147i16, 192i16, -165i16,
    -354i16, -334i16, -106i16, -156i16, -40i16, -3i16, -68i16, 124i16, -257i16, 78i16, 124i16,
    170i16, 412i16, 227i16, 105i16, -104i16, 12i16, 154i16, 250i16, 274i16, 258i16, 4i16, -27i16,
    235i16, 152i16, 51i16, 338i16, 300i16, 7i16, -314i16, -411i16, 215i16, 170i16, -9i16, -93i16,
    -77i16, 76i16, 67i16, 54i16, 200i16, 315i16, 163i16, 72i16, -91i16, -402i16, 158i16, 187i16,
    -156i16, -91i16, 290i16, 267i16, 167i16, 91i16, 140i16, 171i16, 112i16, 9i16, -42i16, -177i16,
    -440i16, 385i16, 80i16, 15i16, 172i16, 129i16, 41i16, -129i16, -372i16, -24i16, -75i16, -30i16,
    -170i16, 10i16, -118i16, 57i16, 78i16, -101i16, 232i16, 161i16, 123i16, 256i16, 277i16, 101i16,
    -192i16, -629i16, -100i16, -60i16, -232i16, 66i16, 13i16, -13i16, -80i16, -239i16, 239i16,
    37i16, 32i16, 89i16, -319i16, -579i16, 450i16, 360i16, 3i16, -29i16, -299i16, -89i16, -54i16,
    -110i16, -246i16, -164i16, 6i16, -188i16, 338i16, 176i16, -92i16, 197i16, 137i16, 134i16,
    12i16, -2i16, 56i16, -183i16, 114i16, -36i16, -131i16, -204i16, 75i16, -25i16, -174i16, 191i16,
    -15i16, -290i16, -429i16, -267i16, 79i16, 37i16, 106i16, 23i16, -384i16, 425i16, 70i16, -14i16,
    212i16, 105i16, 15i16, -2i16, -42i16, -37i16, -123i16, 108i16, 28i16, -48i16, 193i16, 197i16,
    173i16, -33i16, 37i16, 73i16, -57i16, 256i16, 137i16, -58i16, -430i16, -228i16, 217i16, -51i16,
    -10i16, -58i16, -6i16, 22i16, 104i16, 61i16, -119i16, 169i16, 144i16, 16i16, -46i16, -394i16,
    60i16, 454i16, -80i16, -298i16, -65i16, 25i16, 0i16, -24i16, -65i16, -417i16, 465i16, 276i16,
    -3i16, -194i16, -13i16, 130i16, 19i16, -6i16, -21i16, -24i16, -180i16, -53i16, -85i16, 20i16,
    118i16, 147i16, 113i16, -75i16, -289i16, 226i16, -122i16, 227i16, 270i16, 125i16, 109i16,
    197i16, 125i16, 138i16, 44i16, 60i16, 25i16, -55i16, -167i16, -32i16, -139i16, -193i16,
    -173i16, -316i16, 287i16, -208i16, 253i16, 239i16, 27i16, -80i16, -188i16, -28i16, -182i16,
    -235i16, 156i16, -117i16, 128i16, -48i16, -58i16, -226i16, 172i16, 181i16, 167i16, 19i16,
    62i16, 10i16, 2i16, 181i16, 151i16, 108i16, -16i16, -11i16, -78i16, -331i16, 411i16, 133i16,
    17i16, 104i16, 64i16, -184i16, 24i16, -30i16, -3i16, -283i16, 121i16, 204i16, -8i16, -199i16,
    -21i16, -80i16, -169i16, -157i16, -191i16, -136i16, 81i16, 155i16, 14i16, -131i16, 244i16,
    74i16, -57i16, -47i16, -280i16, 347i16, 111i16, -77i16, -128i16, -142i16, -194i16, -125i16,
    -6i16, -68i16, 91i16, 1i16, 23i16, 14i16, -154i16, -34i16, 23i16, -38i16, -343i16, 503i16,
    146i16, -38i16, -46i16, -41i16, 58i16, 31i16, 63i16, -48i16, -117i16, 45i16, 28i16, 1i16,
    -89i16, -5i16, -44i16, -29i16, -448i16, 487i16, 204i16, 81i16, 46i16, -106i16, -302i16, 380i16,
    120i16, -38i16, -12i16, -39i16, 70i16, -3i16, 25i16, -65i16, 30i16, -11i16, 34i16, -15i16,
    22i16, -115i16, 0i16, -79i16, -83i16, 45i16, 114i16, 43i16, 150i16, 36i16, 233i16, 149i16,
    195i16, 5i16, 25i16, -52i16, -475i16, 274i16, 28i16, -39i16, -8i16, -66i16, -255i16, 258i16,
    56i16, 143i16, -45i16, -190i16, 165i16, -60i16, 20i16, 2i16, 125i16, -129i16, 51i16, -8i16,
    -335i16, 288i16, 38i16, 59i16, 25i16, -42i16, 23i16, -118i16, -112i16, 11i16, -55i16, -133i16,
    -109i16, 24i16, -105i16, 78i16, -64i16, -245i16, 202i16, -65i16, -127i16, 162i16, 40i16,
    -94i16, 89i16, -85i16, -119i16, -103i16, 97i16, 9i16, -70i16, -28i16, 194i16, 86i16, -112i16,
    -92i16, -114i16, 74i16, -49i16, 46i16, -84i16, -178i16, 113i16, 52i16, -205i16, 333i16, 88i16,
    222i16, 56i16, -55i16, 13i16, 86i16, 4i16, -77i16, 224i16, 114i16, -105i16, 112i16, 125i16,
    -29i16, -18i16, -144i16, 22i16, -58i16, -99i16, 28i16, 114i16, -66i16, -32i16, -169i16,
    -314i16, 285i16, 72i16, -74i16, 179i16, 28i16, -79i16, -182i16, 13i16, -55i16, 147i16, 13i16,
    12i16, -54i16, 31i16, -84i16, -17i16, -75i16, -228i16, 83i16, -375i16, 436i16, 110i16, -63i16,
    -27i16, -136i16, 169i16, -56i16, -8i16, -171i16, 184i16, -42i16, 148i16, 68i16, 204i16, 235i16,
    110i16, -229i16, 91i16, 171i16, -43i16, -3i16, -26i16, -99i16, -111i16, 71i16, -170i16, 202i16,
    -67i16, 181i16, -37i16, 109i16, -120i16, 3i16, -55i16, -260i16, -16i16, 152i16, 91i16, 142i16,
    42i16, 44i16, 134i16, 47i16, 17i16, -35i16, 22i16, 79i16, -169i16, 41i16, 46i16, 277i16,
    -93i16, -49i16, -126i16, 37i16, -103i16, -34i16, -22i16, -90i16, -134i16, -205i16, 92i16,
    -9i16, 1i16, -195i16, -239i16, 45i16, 54i16, 18i16, -23i16, -1i16, -80i16, -98i16, -20i16,
    -261i16, 306i16, 72i16, 20i16, -89i16, -217i16, 11i16, 6i16, -82i16, 89i16, 13i16, -129i16,
    -89i16, 83i16, -71i16, -55i16, 130i16, -98i16, -146i16, -27i16, -57i16, 53i16, 275i16, 17i16,
    170i16, -5i16, -54i16, 132i16, -64i16, 72i16, 160i16, -125i16, -168i16, 72i16, 40i16, 170i16,
    78i16, 248i16, 116i16, 20i16, 84i16, 31i16, -34i16, 190i16, 38i16, 13i16, -106i16, 225i16,
    27i16, -168i16, 24i16, -157i16, -122i16, 165i16, 11i16, -161i16, -213i16, -12i16, -51i16,
    -101i16, 42i16, 101i16, 27i16, 55i16, 111i16, 75i16, 71i16, -96i16, -1i16, 65i16, -277i16,
    393i16, -26i16, -44i16, -68i16, -84i16, -66i16, -95i16, 235i16, 179i16, -25i16, -41i16, 27i16,
    -91i16, -128i16, -222i16, 146i16, -72i16, -30i16, -24i16, 55i16, -126i16, -68i16, -58i16,
    -127i16, 13i16, -97i16, -106i16, 174i16, -100i16, 155i16, 101i16, -146i16, -21i16, 261i16,
    22i16, 38i16, -66i16, 65i16, 4i16, 70i16, 64i16, 144i16, 59i16, 213i16, 71i16, -337i16, 303i16,
    -52i16, 51i16, -56i16, 1i16, 10i16, -15i16, -5i16, 34i16, 52i16, 228i16, 131i16, 161i16,
    -127i16, -214i16, 238i16, 123i16, 64i16, -147i16, -50i16, -34i16, -127i16, 204i16, 162i16,
    85i16, 41i16, 5i16, -140i16, 73i16, -150i16, 56i16, -96i16, -66i16, -20i16, 2i16, -235i16,
    59i16, -22i16, -107i16, 150i16, -16i16, -47i16, -4i16, 81i16, -67i16, 167i16, 149i16, 149i16,
    -157i16, 288i16, -156i16, -27i16, -8i16, 18i16, 83i16, -24i16, -41i16, -167i16, 158i16,
    -100i16, 93i16, 53i16, 201i16, 15i16, 42i16, 266i16, 278i16, -12i16, -6i16, -37i16, 85i16,
    6i16, 20i16, -188i16, -271i16, 107i16, -13i16, -80i16, 51i16, 202i16, 173i16, -69i16, 78i16,
    -188i16, 46i16, 4i16, 153i16, 12i16, -138i16, 169i16, 5i16, -58i16, -123i16, -108i16, -243i16,
    150i16, 10i16, -191i16, 246i16, -15i16, 38i16, 25i16, -10i16, 14i16, 61i16, 50i16, -206i16,
    -215i16, -220i16, 90i16, 5i16, -149i16, -219i16, 56i16, 142i16, 24i16, -376i16, 77i16, -80i16,
    75i16, 6i16, 42i16, -101i16, 16i16, 56i16, 14i16, -57i16, 3i16, -17i16, 80i16, 57i16, -36i16,
    88i16, -59i16, -97i16, -19i16, -148i16, 46i16, -219i16, 226i16, 114i16, -4i16, -72i16, -15i16,
    37i16, -49i16, -28i16, 247i16, 44i16, 123i16, 47i16, -122i16, -38i16, 17i16, 4i16, -113i16,
    -32i16, -224i16, 154i16, -134i16, 196i16, 71i16, -267i16, -85i16, 28i16, -70i16, 89i16,
    -120i16, 99i16, -2i16, 64i16, 76i16, -166i16, -48i16, 189i16, -35i16, -92i16, -169i16, -123i16,
    339i16, 38i16, -25i16, 38i16, -35i16, 225i16, -139i16, -50i16, -63i16, 246i16, 60i16, -185i16,
    -109i16, -49i16, -53i16, -167i16, 51i16, 149i16, 60i16, -101i16, -33i16, 25i16, -76i16, 120i16,
    32i16, -30i16, -83i16, 102i16, 91i16, -186i16, -261i16, 131i16, -197i16,
];

pub static SKP_SILK_NLSF_CB0_16_STAGE_INFO: Lazy<Vec<SKP_Silk_NLSF_CBS>> = Lazy::new(|| {
    vec![
        SKP_Silk_NLSF_CBS {
            nVectors: 128,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 0..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[0..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 16,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 128..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[128..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[144 * 16..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[144..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 152..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[152..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 160..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[160..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 168..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[168..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 176..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[176..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 184..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[184..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 192..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[192..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 16,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_Q15[16 * 200..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_16_RATES_Q5[200..],
        },
    ]
});

pub static SKP_SILK_NLSF_CB0_16: SKP_Silk_NLSF_CB_struct = SKP_Silk_NLSF_CB_struct {
    nStages: 10,
    CBStages: &SKP_SILK_NLSF_CB0_16_STAGE_INFO,
    NDeltaMin_Q15: &SKP_SILK_NLSF_MSVQ_CB0_16_N_DELTA_MIN_Q15,
    CDF: &SKP_SILK_NLSF_MSVQ_CB0_16_CDF,
    StartPtr: &SKP_SILK_NLSF_MSVQ_CB0_16_CDF_START_PTR,
    MiddleIx: &SKP_SILK_NLSF_MSVQ_CB0_16_CDF_MIDDLE_IDX,
};