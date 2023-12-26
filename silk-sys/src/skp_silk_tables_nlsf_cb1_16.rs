use once_cell::sync::Lazy;

use crate::SKP_Silk_NLSF_MSVQ_decode::{SKP_Silk_NLSF_CBS, SKP_Silk_NLSF_CB_struct};

pub static SKP_SILK_NLSF_MSVQ_CB1_16_CDF: [u16; 114] = [
    0u16, 19099u16, 26957u16, 30639u16, 34242u16, 37546u16, 40447u16, 43287u16, 46005u16, 48445u16,
    49865u16, 51284u16, 52673u16, 53975u16, 55221u16, 56441u16, 57267u16, 58025u16, 58648u16,
    59232u16, 59768u16, 60248u16, 60729u16, 61210u16, 61690u16, 62171u16, 62651u16, 63132u16,
    63613u16, 64093u16, 64574u16, 65054u16, 65535u16, 0u16, 28808u16, 38775u16, 46801u16, 51785u16,
    55886u16, 59410u16, 62572u16, 65535u16, 0u16, 27376u16, 38639u16, 45052u16, 51465u16, 55448u16,
    59021u16, 62594u16, 65535u16, 0u16, 33403u16, 39569u16, 45102u16, 49961u16, 54047u16, 57959u16,
    61788u16, 65535u16, 0u16, 25851u16, 43356u16, 47828u16, 52204u16, 55964u16, 59413u16, 62507u16,
    65535u16, 0u16, 34277u16, 40337u16, 45432u16, 50311u16, 54326u16, 58171u16, 61853u16, 65535u16,
    0u16, 33538u16, 39865u16, 45302u16, 50076u16, 54549u16, 58478u16, 62159u16, 65535u16, 0u16,
    27445u16, 35258u16, 40665u16, 46072u16, 51362u16, 56540u16, 61086u16, 65535u16, 0u16, 22080u16,
    30779u16, 37065u16, 43085u16, 48849u16, 54613u16, 60133u16, 65535u16, 0u16, 13417u16, 21748u16,
    30078u16, 38231u16, 46383u16, 53091u16, 59515u16, 65535u16,
];

pub static SKP_SILK_NLSF_MSVQ_CB1_16_CDF_START_PTR: Lazy<Vec<&[u16]>> = Lazy::new(|| {
    vec![
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[0..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[33..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[42..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[51..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[60..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[69..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[78..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[87..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[96..],
        &SKP_SILK_NLSF_MSVQ_CB1_16_CDF[105..],
    ]
});

pub static SKP_SILK_NLSF_MSVQ_CB1_16_CDF_MIDDLE_IDX: [i32; 10] = [5, 2, 2, 2, 2, 2, 2, 3, 3, 4];

pub static SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5: [i16; 104] = [
    57i16, 98i16, 133i16, 134i16, 138i16, 144i16, 145i16, 147i16, 152i16, 177i16, 177i16, 178i16,
    181i16, 183i16, 184i16, 202i16, 206i16, 215i16, 218i16, 222i16, 227i16, 227i16, 227i16, 227i16,
    227i16, 227i16, 227i16, 227i16, 227i16, 227i16, 227i16, 227i16, 38i16, 87i16, 97i16, 119i16,
    128i16, 135i16, 140i16, 143i16, 40i16, 81i16, 107i16, 107i16, 129i16, 134i16, 134i16, 143i16,
    31i16, 109i16, 114i16, 120i16, 128i16, 130i16, 131i16, 132i16, 43i16, 61i16, 124i16, 125i16,
    132i16, 136i16, 141i16, 142i16, 30i16, 110i16, 118i16, 120i16, 129i16, 131i16, 133i16, 133i16,
    31i16, 108i16, 115i16, 121i16, 124i16, 130i16, 133i16, 137i16, 40i16, 98i16, 115i16, 115i16,
    116i16, 117i16, 123i16, 124i16, 50i16, 93i16, 108i16, 110i16, 112i16, 112i16, 114i16, 115i16,
    73i16, 95i16, 95i16, 96i16, 96i16, 105i16, 107i16, 110i16,
];

pub static SKP_SILK_NLSF_MSVQ_CB1_16_N_DELTA_MIN_Q15: [i32; 17] = [
    148, 3, 60, 68, 117, 86, 121, 124, 152, 153, 207, 151, 225, 239, 126, 183, 792,
];

pub static SKP_SILK_NLSF_MSVQ_CB1_16_Q15: [i16; 1664] = [
    1309i16, 3060i16, 5071i16, 6996i16, 9028i16, 10938i16, 12934i16, 14891i16, 16933i16, 18854i16,
    20792i16, 22764i16, 24753i16, 26659i16, 28626i16, 30501i16, 1264i16, 2745i16, 4610i16, 6408i16,
    8286i16, 10043i16, 12084i16, 14108i16, 16118i16, 18163i16, 20095i16, 22164i16, 24264i16,
    26316i16, 28329i16, 30251i16, 1044i16, 2080i16, 3672i16, 5179i16, 7140i16, 9100i16, 11070i16,
    13065i16, 15423i16, 17790i16, 19931i16, 22101i16, 24290i16, 26361i16, 28499i16, 30418i16,
    1131i16, 2476i16, 4478i16, 6149i16, 7902i16, 9875i16, 11938i16, 13809i16, 15869i16, 17730i16,
    19948i16, 21707i16, 23761i16, 25535i16, 27426i16, 28917i16, 1040i16, 2004i16, 4026i16, 6100i16,
    8432i16, 10494i16, 12610i16, 14694i16, 16797i16, 18775i16, 20799i16, 22782i16, 24772i16,
    26682i16, 28631i16, 30516i16, 2310i16, 3812i16, 5913i16, 7933i16, 10033i16, 11881i16, 13885i16,
    15798i16, 17751i16, 19576i16, 21482i16, 23276i16, 25157i16, 27010i16, 28833i16, 30623i16,
    1254i16, 2847i16, 5013i16, 6781i16, 8626i16, 10370i16, 12726i16, 14633i16, 16281i16, 17852i16,
    19870i16, 21472i16, 23002i16, 24629i16, 26710i16, 27960i16, 1468i16, 3059i16, 4987i16, 7026i16,
    8741i16, 10412i16, 12281i16, 14020i16, 15970i16, 17723i16, 19640i16, 21522i16, 23472i16,
    25661i16, 27986i16, 30225i16, 2171i16, 3566i16, 5605i16, 7384i16, 9404i16, 11220i16, 13030i16,
    14758i16, 16687i16, 18417i16, 20346i16, 22091i16, 24055i16, 26212i16, 28356i16, 30397i16,
    2409i16, 4676i16, 7543i16, 9786i16, 11419i16, 12935i16, 14368i16, 15653i16, 17366i16, 18943i16,
    20762i16, 22477i16, 24440i16, 26327i16, 28284i16, 30242i16, 2354i16, 4222i16, 6820i16, 9107i16,
    11596i16, 13934i16, 15973i16, 17682i16, 19158i16, 20517i16, 21991i16, 23420i16, 25178i16,
    26936i16, 28794i16, 30527i16, 1323i16, 2414i16, 4184i16, 6039i16, 7534i16, 9398i16, 11099i16,
    13097i16, 14799i16, 16451i16, 18434i16, 20887i16, 23490i16, 25838i16, 28046i16, 30225i16,
    1361i16, 3243i16, 6048i16, 8511i16, 11001i16, 13145i16, 15073i16, 16608i16, 18126i16, 19381i16,
    20912i16, 22607i16, 24660i16, 26668i16, 28663i16, 30566i16, 1216i16, 2648i16, 5901i16, 8422i16,
    10037i16, 11425i16, 12973i16, 14603i16, 16686i16, 18600i16, 20555i16, 22415i16, 24450i16,
    26280i16, 28206i16, 30077i16, 2417i16, 4048i16, 6316i16, 8433i16, 10510i16, 12757i16, 15072i16,
    17295i16, 19573i16, 21503i16, 23329i16, 24782i16, 26235i16, 27689i16, 29214i16, 30819i16,
    1012i16, 2345i16, 4991i16, 7377i16, 9465i16, 11916i16, 14296i16, 16566i16, 18672i16, 20544i16,
    22292i16, 23838i16, 25415i16, 27050i16, 28848i16, 30551i16, 1937i16, 3693i16, 6267i16, 8019i16,
    10372i16, 12194i16, 14287i16, 15657i16, 17431i16, 18864i16, 20769i16, 22206i16, 24037i16,
    25463i16, 27383i16, 28602i16, 1969i16, 3305i16, 5017i16, 6726i16, 8375i16, 9993i16, 11634i16,
    13280i16, 15078i16, 16751i16, 18464i16, 20119i16, 21959i16, 23858i16, 26224i16, 29298i16,
    1198i16, 2647i16, 5428i16, 7423i16, 9775i16, 12155i16, 14665i16, 16344i16, 18121i16, 19790i16,
    21557i16, 22847i16, 24484i16, 25742i16, 27639i16, 28711i16, 1636i16, 3353i16, 5447i16, 7597i16,
    9837i16, 11647i16, 13964i16, 16019i16, 17862i16, 20116i16, 22319i16, 24037i16, 25966i16,
    28086i16, 29914i16, 31294i16, 2676i16, 4105i16, 6378i16, 8223i16, 10058i16, 11549i16, 13072i16,
    14453i16, 15956i16, 17355i16, 18931i16, 20402i16, 22183i16, 23884i16, 25717i16, 27723i16,
    1373i16, 2593i16, 4449i16, 5633i16, 7300i16, 8425i16, 9474i16, 10818i16, 12769i16, 15722i16,
    19002i16, 21429i16, 23682i16, 25924i16, 28135i16, 30333i16, 1596i16, 3183i16, 5378i16, 7164i16,
    8670i16, 10105i16, 11470i16, 12834i16, 13991i16, 15042i16, 16642i16, 17903i16, 20759i16,
    25283i16, 27770i16, 30240i16, 2037i16, 3987i16, 6237i16, 8117i16, 9954i16, 12245i16, 14217i16,
    15892i16, 17775i16, 20114i16, 22314i16, 25942i16, 26305i16, 26483i16, 26796i16, 28561i16,
    2181i16, 3858i16, 5760i16, 7924i16, 10041i16, 11577i16, 13769i16, 15700i16, 17429i16, 19879i16,
    23583i16, 24538i16, 25212i16, 25693i16, 28688i16, 30507i16, 1992i16, 3882i16, 6474i16, 7883i16,
    9381i16, 12672i16, 14340i16, 15701i16, 16658i16, 17832i16, 20850i16, 22885i16, 24677i16,
    26457i16, 28491i16, 30460i16, 2391i16, 3988i16, 5448i16, 7432i16, 11014i16, 12579i16, 13140i16,
    14146i16, 15898i16, 18592i16, 21104i16, 22993i16, 24673i16, 27186i16, 28142i16, 29612i16,
    1713i16, 5102i16, 6989i16, 7798i16, 8670i16, 10110i16, 12746i16, 14881i16, 16709i16, 18407i16,
    20126i16, 22107i16, 24181i16, 26198i16, 28237i16, 30137i16, 1612i16, 3617i16, 6148i16, 8359i16,
    9576i16, 11528i16, 14936i16, 17809i16, 18287i16, 18729i16, 19001i16, 21111i16, 24631i16,
    26596i16, 28740i16, 30643i16, 2266i16, 4168i16, 7862i16, 9546i16, 9618i16, 9703i16, 10134i16,
    13897i16, 16265i16, 18432i16, 20587i16, 22605i16, 24754i16, 26994i16, 29125i16, 30840i16,
    1840i16, 3917i16, 6272i16, 7809i16, 9714i16, 11438i16, 13767i16, 15799i16, 19244i16, 21972i16,
    22980i16, 23180i16, 23723i16, 25650i16, 29117i16, 31085i16, 1458i16, 3612i16, 6008i16, 7488i16,
    9827i16, 11893i16, 14086i16, 15734i16, 17440i16, 19535i16, 22424i16, 24767i16, 29246i16,
    29928i16, 30516i16, 30947i16, -102i16, -121i16, -31i16, -6i16, 5i16, -2i16, 8i16, -18i16,
    -4i16, 6i16, 14i16, -2i16, -12i16, -16i16, -12i16, -60i16, -126i16, -353i16, -574i16, -677i16,
    -657i16, -617i16, -498i16, -393i16, -348i16, -277i16, -225i16, -164i16, -102i16, -70i16,
    -31i16, 33i16, 4i16, 379i16, 387i16, 551i16, 605i16, 620i16, 532i16, 482i16, 442i16, 454i16,
    385i16, 347i16, 322i16, 299i16, 266i16, 200i16, 1168i16, 951i16, 672i16, 246i16, 60i16,
    -161i16, -259i16, -234i16, -253i16, -282i16, -203i16, -187i16, -155i16, -176i16, -198i16,
    -178i16, 10i16, 170i16, 393i16, 609i16, 555i16, 208i16, -330i16, -571i16, -769i16, -633i16,
    -319i16, -43i16, 95i16, 105i16, 106i16, 116i16, -152i16, -140i16, -125i16, 5i16, 173i16,
    274i16, 264i16, 331i16, -37i16, -293i16, -609i16, -786i16, -959i16, -814i16, -645i16, -238i16,
    -91i16, 36i16, -11i16, -101i16, -279i16, -227i16, -40i16, 90i16, 530i16, 677i16, 890i16,
    1104i16, 999i16, 835i16, 564i16, 295i16, -280i16, -364i16, -340i16, -331i16, -284i16, 288i16,
    761i16, 880i16, 988i16, 627i16, 146i16, -226i16, -203i16, -181i16, -142i16, 39i16, 24i16,
    -26i16, -107i16, -92i16, -161i16, -135i16, -131i16, -88i16, -160i16, -156i16, -75i16, -43i16,
    -36i16, -6i16, -33i16, 33i16, -324i16, -415i16, -108i16, 124i16, 157i16, 191i16, 203i16,
    197i16, 144i16, 109i16, 152i16, 176i16, 190i16, 122i16, 101i16, 159i16, 663i16, 668i16, 480i16,
    400i16, 379i16, 444i16, 446i16, 458i16, 343i16, 351i16, 310i16, 228i16, 133i16, 44i16, 75i16,
    63i16, -84i16, 39i16, -29i16, 35i16, -94i16, -233i16, -261i16, -354i16, 77i16, 262i16, -24i16,
    -145i16, -333i16, -409i16, -404i16, -597i16, -488i16, -300i16, 910i16, 592i16, 412i16, 120i16,
    130i16, -51i16, -37i16, -77i16, -172i16, -181i16, -159i16, -148i16, -72i16, -62i16, 510i16,
    516i16, 113i16, -585i16, -1075i16, -957i16, -417i16, -195i16, 9i16, 7i16, -88i16, -173i16,
    -91i16, 54i16, 98i16, 95i16, -28i16, 197i16, -527i16, -621i16, 157i16, 122i16, -168i16, 147i16,
    309i16, 300i16, 336i16, 315i16, 396i16, 408i16, 376i16, 106i16, -162i16, -170i16, -315i16,
    98i16, 821i16, 908i16, 570i16, -33i16, -312i16, -568i16, -572i16, -378i16, -107i16, 23i16,
    156i16, 93i16, -129i16, -87i16, 20i16, -72i16, -37i16, 40i16, 21i16, 27i16, 48i16, 75i16,
    77i16, 65i16, 46i16, 71i16, 66i16, 47i16, 136i16, 344i16, 236i16, 322i16, 170i16, 283i16,
    269i16, 291i16, 162i16, -43i16, -204i16, -259i16, -240i16, -305i16, -350i16, -312i16, 447i16,
    348i16, 345i16, 257i16, 71i16, -131i16, -77i16, -190i16, -202i16, -40i16, 35i16, 133i16,
    261i16, 365i16, 438i16, 303i16, -8i16, 22i16, 140i16, 137i16, -300i16, -641i16, -764i16,
    -268i16, -23i16, -25i16, 73i16, -162i16, -150i16, -212i16, -72i16, 6i16, 39i16, 78i16, 104i16,
    -93i16, -308i16, -136i16, 117i16, -71i16, -513i16, -820i16, -700i16, -450i16, -161i16, -23i16,
    29i16, 78i16, 337i16, 106i16, -406i16, -782i16, -112i16, 233i16, 383i16, 62i16, -126i16, 6i16,
    -77i16, -29i16, -146i16, -123i16, -51i16, -27i16, -27i16, -381i16, -641i16, 402i16, 539i16,
    8i16, -207i16, -366i16, -36i16, -27i16, -204i16, -227i16, -237i16, -189i16, -64i16, 51i16,
    -92i16, -137i16, -281i16, 62i16, 233i16, 92i16, 148i16, 294i16, 363i16, 416i16, 564i16, 625i16,
    370i16, -36i16, -469i16, -462i16, 102i16, 168i16, 32i16, 117i16, -21i16, 97i16, 139i16, 89i16,
    104i16, 35i16, 4i16, 82i16, 66i16, 58i16, 73i16, 93i16, -76i16, -320i16, -236i16, -189i16,
    -203i16, -142i16, -27i16, -73i16, 9i16, -9i16, -25i16, 12i16, -15i16, 4i16, 4i16, -50i16,
    314i16, 180i16, 162i16, -49i16, 199i16, -108i16, -227i16, -66i16, -447i16, -67i16, -264i16,
    -394i16, 5i16, 55i16, -133i16, -176i16, -116i16, -241i16, 272i16, 109i16, 282i16, 262i16,
    192i16, -64i16, -392i16, -514i16, 156i16, 203i16, 154i16, 72i16, -34i16, -160i16, -73i16, 3i16,
    -33i16, -431i16, 321i16, 18i16, -567i16, -590i16, -108i16, 88i16, 66i16, 51i16, -31i16,
    -193i16, -46i16, 65i16, -29i16, -23i16, 215i16, -31i16, 101i16, -113i16, 32i16, 304i16, 88i16,
    320i16, 448i16, 5i16, -439i16, -562i16, -508i16, -135i16, -13i16, -171i16, -8i16, 182i16,
    -99i16, -181i16, -149i16, 376i16, 476i16, 64i16, -396i16, -652i16, -150i16, 176i16, 222i16,
    65i16, -590i16, 719i16, 271i16, 399i16, 245i16, 72i16, -156i16, -152i16, -176i16, 59i16, 94i16,
    125i16, -9i16, -7i16, 9i16, 1i16, -61i16, -116i16, -82i16, 1i16, 79i16, 22i16, -44i16, -15i16,
    -48i16, -65i16, -62i16, -101i16, -102i16, -54i16, -70i16, -78i16, -80i16, -25i16, 398i16,
    71i16, 139i16, 38i16, 90i16, 194i16, 222i16, 249i16, 165i16, 94i16, 221i16, 262i16, 163i16,
    91i16, -206i16, 573i16, 200i16, -287i16, -147i16, 5i16, -18i16, -85i16, -74i16, -125i16,
    -87i16, 85i16, 141i16, 4i16, -4i16, 28i16, 234i16, 48i16, -150i16, -111i16, -506i16, 237i16,
    -209i16, 345i16, 94i16, -124i16, 77i16, 121i16, 143i16, 12i16, -80i16, -48i16, 191i16, 144i16,
    -93i16, -65i16, -151i16, -643i16, 435i16, 106i16, 87i16, 7i16, 65i16, 102i16, 94i16, 68i16,
    5i16, 99i16, 222i16, 93i16, 94i16, 355i16, -13i16, -89i16, -228i16, -503i16, 287i16, 109i16,
    108i16, 449i16, 253i16, -29i16, -109i16, -116i16, 15i16, -73i16, -20i16, 131i16, -147i16,
    72i16, 59i16, -150i16, -594i16, 273i16, 316i16, 132i16, 199i16, 106i16, 198i16, 212i16, 220i16,
    82i16, 45i16, -13i16, 223i16, 137i16, 270i16, 38i16, 252i16, 135i16, -177i16, -207i16, -360i16,
    -102i16, 403i16, 406i16, -14i16, 83i16, 64i16, 51i16, -7i16, -99i16, -97i16, -88i16, -124i16,
    -65i16, 42i16, 32i16, 28i16, 29i16, 12i16, 20i16, 119i16, -26i16, -212i16, -201i16, 373i16,
    251i16, 141i16, 103i16, 36i16, -52i16, 66i16, 18i16, -6i16, -95i16, -196i16, 5i16, 98i16,
    -85i16, -108i16, 218i16, -164i16, 20i16, 356i16, 172i16, 37i16, 266i16, 23i16, 112i16, -24i16,
    -99i16, -92i16, -178i16, 29i16, -278i16, 388i16, -60i16, -220i16, 300i16, -13i16, 154i16,
    191i16, 15i16, -37i16, -110i16, -153i16, -150i16, -114i16, -7i16, -94i16, -31i16, -62i16,
    -177i16, 4i16, -70i16, 35i16, 453i16, 147i16, -247i16, -328i16, 101i16, 20i16, -114i16, 147i16,
    108i16, -119i16, -109i16, -102i16, -238i16, 55i16, -102i16, 173i16, -89i16, 129i16, 138i16,
    -330i16, -160i16, 485i16, 154i16, -59i16, -170i16, -20i16, -34i16, -261i16, -40i16, -129i16,
    77i16, -84i16, 69i16, 83i16, 160i16, 169i16, 63i16, -516i16, 30i16, 336i16, 52i16, -0i16,
    -52i16, -124i16, 158i16, 19i16, 197i16, -10i16, -375i16, 405i16, 285i16, 114i16, -395i16,
    -47i16, 196i16, 62i16, 87i16, -106i16, -65i16, -75i16, -69i16, -13i16, 34i16, 99i16, 59i16,
    83i16, 98i16, 44i16, 0i16, 24i16, 18i16, 17i16, 70i16, -22i16, 194i16, 208i16, 144i16, -79i16,
    -15i16, 32i16, -104i16, -28i16, -105i16, -186i16, -212i16, -228i16, -79i16, -76i16, 51i16,
    -71i16, 72i16, 118i16, -34i16, -3i16, -171i16, 5i16, 2i16, -108i16, -125i16, 62i16, -58i16,
    58i16, -121i16, 73i16, -466i16, 92i16, 63i16, -94i16, -78i16, -76i16, 212i16, 36i16, -225i16,
    -71i16, -354i16, 152i16, 143i16, -79i16, -246i16, -51i16, -31i16, -6i16, -270i16, 240i16,
    210i16, 30i16, -157i16, -231i16, 74i16, -146i16, 88i16, -273i16, 156i16, 92i16, 56i16, 71i16,
    2i16, 318i16, 164i16, 32i16, -110i16, -35i16, -41i16, -95i16, -106i16, 11i16, 132i16, -68i16,
    55i16, 123i16, -83i16, -149i16, 212i16, 132i16, 0i16, -194i16, 55i16, 206i16, -108i16, -353i16,
    289i16, -195i16, 1i16, 233i16, -22i16, -60i16, 20i16, 26i16, 68i16, 166i16, 27i16, -58i16,
    130i16, 112i16, 107i16, 27i16, -165i16, 115i16, -93i16, -37i16, 38i16, 83i16, 483i16, 65i16,
    -229i16, -13i16, 157i16, 85i16, 50i16, 136i16, 10i16, 32i16, 83i16, 82i16, 55i16, 5i16, -9i16,
    -52i16, -78i16, -81i16, -51i16, 40i16, 18i16, -127i16, -224i16, -41i16, 53i16, -210i16,
    -113i16, 24i16, -17i16, -187i16, -89i16, 8i16, 121i16, 83i16, 77i16, 91i16, -74i16, -35i16,
    -112i16, -161i16, -173i16, 102i16, 132i16, -125i16, -61i16, 103i16, -260i16, 52i16, 166i16,
    -32i16, -156i16, -87i16, -56i16, 60i16, -70i16, -124i16, 242i16, 114i16, -251i16, -166i16,
    201i16, 127i16, 28i16, -11i16, 23i16, -80i16, -115i16, -20i16, -51i16, -348i16, 340i16, -34i16,
    133i16, 13i16, 92i16, -124i16, -136i16, -120i16, -26i16, -6i16, 17i16, 28i16, 21i16, 120i16,
    -168i16, 160i16, -35i16, 115i16, 28i16, 9i16, 7i16, -56i16, 39i16, 156i16, 256i16, -18i16,
    1i16, 277i16, 82i16, -70i16, -144i16, -88i16, -13i16, -59i16, -157i16, 8i16, -134i16, 21i16,
    -40i16, 58i16, -21i16, 194i16, -276i16, 97i16, 279i16, -56i16, -140i16, 125i16, 57i16, -184i16,
    -204i16, -70i16, -2i16, 128i16, -202i16, -78i16, 230i16, -23i16, 161i16, -102i16, 1i16, 1i16,
    180i16, -31i16, -86i16, -167i16, -57i16, -60i16, 27i16, -13i16, 99i16, 108i16, 111i16, 76i16,
    69i16, 34i16, -21i16, 53i16, 38i16, 34i16, 78i16, 73i16, 219i16, 51i16, 15i16, -72i16, -103i16,
    -207i16, 30i16, 213i16, -14i16, 31i16, -94i16, -40i16, -144i16, 67i16, 4i16, 105i16, 59i16,
    -240i16, 25i16, 244i16, 69i16, 58i16, 23i16, -24i16, -5i16, -15i16, -133i16, -71i16, -67i16,
    181i16, 29i16, -45i16, 121i16, 96i16, 51i16, -72i16, -53i16, 56i16, -153i16, -27i16, 85i16,
    183i16, 211i16, 105i16, -34i16, -46i16, 43i16, -72i16, -93i16, 36i16, -128i16, 29i16, 111i16,
    -95i16, -156i16, -179i16, -235i16, 21i16, -39i16, -71i16, -33i16, -61i16, -252i16, 230i16,
    -131i16, 157i16, -21i16, -85i16, -28i16, -123i16, 80i16, -160i16, 63i16, 47i16, -6i16, -49i16,
    -96i16, -19i16, 17i16, -58i16, 17i16, -0i16, -13i16, -170i16, 25i16, -35i16, 59i16, 10i16,
    -31i16, -413i16, 81i16, 62i16, 18i16, -164i16, 245i16, 92i16, -165i16, 42i16, 26i16, 126i16,
    -248i16, 193i16, -55i16, 16i16, 39i16, 14i16, 50i16,
];

pub static SKP_SILK_NLSF_CB1_16_STAGE_INFO: Lazy<Vec<SKP_Silk_NLSF_CBS>> = Lazy::new(|| {
    vec![
        SKP_Silk_NLSF_CBS {
            nVectors: 32,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 0..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[0..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 32..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[32..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 40..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[40..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 48..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[48..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 56..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[56..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 64..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[64..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 72..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[72..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 80..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[80..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 88..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[88..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_Q15[16 * 96..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB1_16_RATES_Q5[96..],
        },
    ]
});

pub static SKP_SILK_NLSF_CB1_16: SKP_Silk_NLSF_CB_struct = SKP_Silk_NLSF_CB_struct {
    nStages: 10,
    CBStages: &SKP_SILK_NLSF_CB1_16_STAGE_INFO,
    NDeltaMin_Q15: &SKP_SILK_NLSF_MSVQ_CB1_16_N_DELTA_MIN_Q15,
    CDF: &SKP_SILK_NLSF_MSVQ_CB1_16_CDF,
    StartPtr: &SKP_SILK_NLSF_MSVQ_CB1_16_CDF_START_PTR,
    MiddleIx: &SKP_SILK_NLSF_MSVQ_CB1_16_CDF_MIDDLE_IDX,
};