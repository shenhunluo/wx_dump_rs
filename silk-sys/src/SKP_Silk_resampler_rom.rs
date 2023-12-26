pub static SKP_SILK_RESAMPLER_DOWN2_0: i16 = 9872i16;

pub static SKP_SILK_RESAMPLER_DOWN2_1: i16 = (39809 - 65536) as i16;

pub static SKP_SILK_RESAMPLER_UP2_LQ_0: i16 = 8102i16;

pub static SKP_SILK_RESAMPLER_UP2_LQ_1: i16 = (36783 - 65536) as i16;

pub static SKP_SILK_RESAMPLER_UP2_HQ_0: [i16; 2] = [4280i16, (33727 - 65536) as i16];

pub static SKP_SILK_RESAMPLER_UP2_HQ_1: [i16; 2] = [16295i16, (54015 - 65536) as i16];

pub static SKP_SILK_RESAMPLER_UP2_HQ_NOTCH: [i16; 4] = [7864i16, -3604i16, 13107i16, 28508i16];

pub static SKP_SILK_RESAMPLER_3_4_COEFS: [i16; 20] = [
    -18249i16, -12532i16, -97i16, 284i16, -495i16, 309i16, 10268i16, 20317i16, -94i16, 156i16,
    -48i16, -720i16, 5984i16, 18278i16, -45i16, -4i16, 237i16, -847i16, 2540i16, 14662i16,
];

pub static SKP_SILK_RESAMPLER_2_3_COEFS: [i16; 14] = [
    -11891i16, -12486i16, 20i16, 211i16, -657i16, 688i16, 8423i16, 15911i16, -44i16, 197i16,
    -152i16, -653i16, 3855i16, 13015i16,
];

pub static SKP_SILK_RESAMPLER_1_2_COEFS: [i16; 8] = [
    2415i16, -13101i16, 158i16, -295i16, -400i16, 1265i16, 4832i16, 7968i16,
];

pub static SKP_SILK_RESAMPLER_3_8_COEFS: [i16; 20] = [
    13270i16, -13738i16, -294i16, -123i16, 747i16, 2043i16, 3339i16, 3995i16, -151i16, -311i16,
    414i16, 1583i16, 2947i16, 3877i16, -33i16, -389i16, 143i16, 1141i16, 2503i16, 3653i16,
];

pub static SKP_SILK_RESAMPLER_1_3_COEFS: [i16; 8] = [
    16643i16, -14000i16, -331i16, 19i16, 581i16, 1421i16, 2290i16, 2845i16,
];

pub static SKP_SILK_RESAMPLER_2_3_COEFS_LQ: [i16; 6] =
    [-2797i16, -6507i16, 4697i16, 10739i16, 1567i16, 8276i16];

pub static SKP_SILK_RESAMPLER_1_3_COEFS_LQ: [i16; 5] =
    [16777i16, -9792i16, 890i16, 1614i16, 2148i16];

pub static SKP_SILK_RESAMPLER_320_441_ARMA4_COEFS: [i16; 7] = [
    31454i16, 24746i16, -9706i16, -3386i16, -17911i16, -13243i16, 24797i16,
];

pub static SKP_SILK_RESAMPLER_240_441_ARMA4_COEFS: [i16; 7] = [
    28721i16, 11254i16, 3189i16, -2546i16, -1495i16, -12618i16, 11562i16,
];

pub static SKP_SILK_RESAMPLER_160_441_ARMA4_COEFS: [i16; 7] = [
    23492i16, -6457i16, 14358i16, -4856i16, 14654i16, -13008i16, 4456i16,
];

pub static SKP_SILK_RESAMPLER_120_441_ARMA4_COEFS: [i16; 7] = [
    19311i16, -15569i16, 19489i16, -6950i16, 21441i16, -13559i16, 2370i16,
];

pub static SKP_SILK_RESAMPLER_80_441_ARMA4_COEFS: [i16; 7] = [
    13248i16, -23849i16, 24126i16, -9486i16, 26806i16, -14286i16, 1065i16,
];

pub static SKP_SILK_RESAMPLER_FRAC_FIR_144: [[i16; 3]; 144] = [
    [-647i16, 1884i16, 30078i16],
    [-625i16, 1736i16, 30044i16],
    [-603i16, 1591i16, 30005i16],
    [-581i16, 1448i16, 29963i16],
    [-559i16, 1308i16, 29917i16],
    [-537i16, 1169i16, 29867i16],
    [-515i16, 1032i16, 29813i16],
    [-494i16, 898i16, 29755i16],
    [-473i16, 766i16, 29693i16],
    [-452i16, 636i16, 29627i16],
    [-431i16, 508i16, 29558i16],
    [-410i16, 383i16, 29484i16],
    [-390i16, 260i16, 29407i16],
    [-369i16, 139i16, 29327i16],
    [-349i16, 20i16, 29242i16],
    [-330i16, -97i16, 29154i16],
    [-310i16, -211i16, 29062i16],
    [-291i16, -324i16, 28967i16],
    [-271i16, -434i16, 28868i16],
    [-253i16, -542i16, 28765i16],
    [-234i16, -647i16, 28659i16],
    [-215i16, -751i16, 28550i16],
    [-197i16, -852i16, 28436i16],
    [-179i16, -951i16, 28320i16],
    [-162i16, -1048i16, 28200i16],
    [-144i16, -1143i16, 28077i16],
    [-127i16, -1235i16, 27950i16],
    [-110i16, -1326i16, 27820i16],
    [-94i16, -1414i16, 27687i16],
    [-77i16, -1500i16, 27550i16],
    [-61i16, -1584i16, 27410i16],
    [-45i16, -1665i16, 27268i16],
    [-30i16, -1745i16, 27122i16],
    [-15i16, -1822i16, 26972i16],
    [0i16, -1897i16, 26820i16],
    [15i16, -1970i16, 26665i16],
    [29i16, -2041i16, 26507i16],
    [44i16, -2110i16, 26346i16],
    [57i16, -2177i16, 26182i16],
    [71i16, -2242i16, 26015i16],
    [84i16, -2305i16, 25845i16],
    [97i16, -2365i16, 25673i16],
    [110i16, -2424i16, 25498i16],
    [122i16, -2480i16, 25320i16],
    [134i16, -2534i16, 25140i16],
    [146i16, -2587i16, 24956i16],
    [157i16, -2637i16, 24771i16],
    [168i16, -2685i16, 24583i16],
    [179i16, -2732i16, 24392i16],
    [190i16, -2776i16, 24199i16],
    [200i16, -2819i16, 24003i16],
    [210i16, -2859i16, 23805i16],
    [220i16, -2898i16, 23605i16],
    [229i16, -2934i16, 23403i16],
    [238i16, -2969i16, 23198i16],
    [247i16, -3002i16, 22992i16],
    [255i16, -3033i16, 22783i16],
    [263i16, -3062i16, 22572i16],
    [271i16, -3089i16, 22359i16],
    [279i16, -3114i16, 22144i16],
    [286i16, -3138i16, 21927i16],
    [293i16, -3160i16, 21709i16],
    [300i16, -3180i16, 21488i16],
    [306i16, -3198i16, 21266i16],
    [312i16, -3215i16, 21042i16],
    [318i16, -3229i16, 20816i16],
    [323i16, -3242i16, 20589i16],
    [328i16, -3254i16, 20360i16],
    [333i16, -3263i16, 20130i16],
    [338i16, -3272i16, 19898i16],
    [342i16, -3278i16, 19665i16],
    [346i16, -3283i16, 19430i16],
    [350i16, -3286i16, 19194i16],
    [353i16, -3288i16, 18957i16],
    [356i16, -3288i16, 18718i16],
    [359i16, -3286i16, 18478i16],
    [362i16, -3283i16, 18238i16],
    [364i16, -3279i16, 17996i16],
    [366i16, -3273i16, 17753i16],
    [368i16, -3266i16, 17509i16],
    [369i16, -3257i16, 17264i16],
    [371i16, -3247i16, 17018i16],
    [372i16, -3235i16, 16772i16],
    [372i16, -3222i16, 16525i16],
    [373i16, -3208i16, 16277i16],
    [373i16, -3192i16, 16028i16],
    [373i16, -3175i16, 15779i16],
    [373i16, -3157i16, 15529i16],
    [372i16, -3138i16, 15279i16],
    [371i16, -3117i16, 15028i16],
    [370i16, -3095i16, 14777i16],
    [369i16, -3072i16, 14526i16],
    [368i16, -3048i16, 14274i16],
    [366i16, -3022i16, 14022i16],
    [364i16, -2996i16, 13770i16],
    [362i16, -2968i16, 13517i16],
    [359i16, -2940i16, 13265i16],
    [357i16, -2910i16, 13012i16],
    [354i16, -2880i16, 12760i16],
    [351i16, -2848i16, 12508i16],
    [348i16, -2815i16, 12255i16],
    [344i16, -2782i16, 12003i16],
    [341i16, -2747i16, 11751i16],
    [337i16, -2712i16, 11500i16],
    [333i16, -2676i16, 11248i16],
    [328i16, -2639i16, 10997i16],
    [324i16, -2601i16, 10747i16],
    [320i16, -2562i16, 10497i16],
    [315i16, -2523i16, 10247i16],
    [310i16, -2482i16, 9998i16],
    [305i16, -2442i16, 9750i16],
    [300i16, -2400i16, 9502i16],
    [294i16, -2358i16, 9255i16],
    [289i16, -2315i16, 9009i16],
    [283i16, -2271i16, 8763i16],
    [277i16, -2227i16, 8519i16],
    [271i16, -2182i16, 8275i16],
    [265i16, -2137i16, 8032i16],
    [259i16, -2091i16, 7791i16],
    [252i16, -2045i16, 7550i16],
    [246i16, -1998i16, 7311i16],
    [239i16, -1951i16, 7072i16],
    [232i16, -1904i16, 6835i16],
    [226i16, -1856i16, 6599i16],
    [219i16, -1807i16, 6364i16],
    [212i16, -1758i16, 6131i16],
    [204i16, -1709i16, 5899i16],
    [197i16, -1660i16, 5668i16],
    [190i16, -1611i16, 5439i16],
    [183i16, -1561i16, 5212i16],
    [175i16, -1511i16, 4986i16],
    [168i16, -1460i16, 4761i16],
    [160i16, -1410i16, 4538i16],
    [152i16, -1359i16, 4317i16],
    [145i16, -1309i16, 4098i16],
    [137i16, -1258i16, 3880i16],
    [129i16, -1207i16, 3664i16],
    [121i16, -1156i16, 3450i16],
    [113i16, -1105i16, 3238i16],
    [105i16, -1054i16, 3028i16],
    [97i16, -1003i16, 2820i16],
    [89i16, -952i16, 2614i16],
    [81i16, -901i16, 2409i16],
    [73i16, -851i16, 2207i16],
];
