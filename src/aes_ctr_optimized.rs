use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::char::from_u32;
use std::convert::TryInto;

// Lookup Tables
const SUBSTITUTION: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];

const MULTIPLY_2: [u8; 256] =
[
0x00,0x02,0x04,0x06,0x08,0x0a,0x0c,0x0e,0x10,0x12,0x14,0x16,0x18,0x1a,0x1c,0x1e,
0x20,0x22,0x24,0x26,0x28,0x2a,0x2c,0x2e,0x30,0x32,0x34,0x36,0x38,0x3a,0x3c,0x3e,
0x40,0x42,0x44,0x46,0x48,0x4a,0x4c,0x4e,0x50,0x52,0x54,0x56,0x58,0x5a,0x5c,0x5e,
0x60,0x62,0x64,0x66,0x68,0x6a,0x6c,0x6e,0x70,0x72,0x74,0x76,0x78,0x7a,0x7c,0x7e,
0x80,0x82,0x84,0x86,0x88,0x8a,0x8c,0x8e,0x90,0x92,0x94,0x96,0x98,0x9a,0x9c,0x9e,
0xa0,0xa2,0xa4,0xa6,0xa8,0xaa,0xac,0xae,0xb0,0xb2,0xb4,0xb6,0xb8,0xba,0xbc,0xbe,
0xc0,0xc2,0xc4,0xc6,0xc8,0xca,0xcc,0xce,0xd0,0xd2,0xd4,0xd6,0xd8,0xda,0xdc,0xde,
0xe0,0xe2,0xe4,0xe6,0xe8,0xea,0xec,0xee,0xf0,0xf2,0xf4,0xf6,0xf8,0xfa,0xfc,0xfe,
0x1b,0x19,0x1f,0x1d,0x13,0x11,0x17,0x15,0x0b,0x09,0x0f,0x0d,0x03,0x01,0x07,0x05,
0x3b,0x39,0x3f,0x3d,0x33,0x31,0x37,0x35,0x2b,0x29,0x2f,0x2d,0x23,0x21,0x27,0x25,
0x5b,0x59,0x5f,0x5d,0x53,0x51,0x57,0x55,0x4b,0x49,0x4f,0x4d,0x43,0x41,0x47,0x45,
0x7b,0x79,0x7f,0x7d,0x73,0x71,0x77,0x75,0x6b,0x69,0x6f,0x6d,0x63,0x61,0x67,0x65,
0x9b,0x99,0x9f,0x9d,0x93,0x91,0x97,0x95,0x8b,0x89,0x8f,0x8d,0x83,0x81,0x87,0x85,
0xbb,0xb9,0xbf,0xbd,0xb3,0xb1,0xb7,0xb5,0xab,0xa9,0xaf,0xad,0xa3,0xa1,0xa7,0xa5,
0xdb,0xd9,0xdf,0xdd,0xd3,0xd1,0xd7,0xd5,0xcb,0xc9,0xcf,0xcd,0xc3,0xc1,0xc7,0xc5,
0xfb,0xf9,0xff,0xfd,0xf3,0xf1,0xf7,0xf5,0xeb,0xe9,0xef,0xed,0xe3,0xe1,0xe7,0xe5
];

const MULTIPLY_3: [u8; 256] =
[
0x00,0x03,0x06,0x05,0x0c,0x0f,0x0a,0x09,0x18,0x1b,0x1e,0x1d,0x14,0x17,0x12,0x11,
0x30,0x33,0x36,0x35,0x3c,0x3f,0x3a,0x39,0x28,0x2b,0x2e,0x2d,0x24,0x27,0x22,0x21,
0x60,0x63,0x66,0x65,0x6c,0x6f,0x6a,0x69,0x78,0x7b,0x7e,0x7d,0x74,0x77,0x72,0x71,
0x50,0x53,0x56,0x55,0x5c,0x5f,0x5a,0x59,0x48,0x4b,0x4e,0x4d,0x44,0x47,0x42,0x41,
0xc0,0xc3,0xc6,0xc5,0xcc,0xcf,0xca,0xc9,0xd8,0xdb,0xde,0xdd,0xd4,0xd7,0xd2,0xd1,
0xf0,0xf3,0xf6,0xf5,0xfc,0xff,0xfa,0xf9,0xe8,0xeb,0xee,0xed,0xe4,0xe7,0xe2,0xe1,
0xa0,0xa3,0xa6,0xa5,0xac,0xaf,0xaa,0xa9,0xb8,0xbb,0xbe,0xbd,0xb4,0xb7,0xb2,0xb1,
0x90,0x93,0x96,0x95,0x9c,0x9f,0x9a,0x99,0x88,0x8b,0x8e,0x8d,0x84,0x87,0x82,0x81,
0x9b,0x98,0x9d,0x9e,0x97,0x94,0x91,0x92,0x83,0x80,0x85,0x86,0x8f,0x8c,0x89,0x8a,
0xab,0xa8,0xad,0xae,0xa7,0xa4,0xa1,0xa2,0xb3,0xb0,0xb5,0xb6,0xbf,0xbc,0xb9,0xba,
0xfb,0xf8,0xfd,0xfe,0xf7,0xf4,0xf1,0xf2,0xe3,0xe0,0xe5,0xe6,0xef,0xec,0xe9,0xea,
0xcb,0xc8,0xcd,0xce,0xc7,0xc4,0xc1,0xc2,0xd3,0xd0,0xd5,0xd6,0xdf,0xdc,0xd9,0xda,
0x5b,0x58,0x5d,0x5e,0x57,0x54,0x51,0x52,0x43,0x40,0x45,0x46,0x4f,0x4c,0x49,0x4a,
0x6b,0x68,0x6d,0x6e,0x67,0x64,0x61,0x62,0x73,0x70,0x75,0x76,0x7f,0x7c,0x79,0x7a,
0x3b,0x38,0x3d,0x3e,0x37,0x34,0x31,0x32,0x23,0x20,0x25,0x26,0x2f,0x2c,0x29,0x2a,
0x0b,0x08,0x0d,0x0e,0x07,0x04,0x01,0x02,0x13,0x10,0x15,0x16,0x1f,0x1c,0x19,0x1a
];

const RCON: [u8; 21] =
[
0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c,	0xd8, 0xab, 0x4d, 0x9a,	0x2f, 0x5e, 0xbc, 0x63, 0xc6
];

const T_0: [u32; 256] = [3328402341, 4168907908, 4000806809, 4135287693, 4294111757, 3597364157, 3731845041, 2445657428, 1613770832, 33620227,
    3462883241, 1445669757, 3892248089, 3050821474, 1303096294, 3967186586, 2412431941, 528646813, 2311702848, 4202528135,
    4026202645, 2992200171, 2387036105, 4226871307, 1101901292, 3017069671, 1604494077, 1169141738, 597466303, 1403299063,
    3832705686, 2613100635, 1974974402, 3791519004, 1033081774, 1277568618, 1815492186, 2118074177, 4126668546, 2211236943,
    1748251740, 1369810420, 3521504564, 4193382664, 3799085459, 2883115123, 1647391059, 706024767, 134480908, 2512897874,
    1176707941, 2646852446, 806885416, 932615841, 168101135, 798661301, 235341577, 605164086, 461406363, 3756188221,
    3454790438, 1311188841, 2142417613, 3933566367, 302582043, 495158174, 1479289972, 874125870, 907746093, 3698224818,
    3025820398, 1537253627, 2756858614, 1983593293, 3084310113, 2108928974, 1378429307, 3722699582, 1580150641, 327451799,
    2790478837, 3117535592, 0, 3253595436, 1075847264, 3825007647, 2041688520, 3059440621, 3563743934, 2378943302,
    1740553945, 1916352843, 2487896798, 2555137236, 2958579944, 2244988746, 3151024235, 3320835882, 1336584933, 3992714006,
    2252555205, 2588757463, 1714631509, 293963156, 2319795663, 3925473552, 67240454, 4269768577, 2689618160, 2017213508,
    631218106, 1269344483, 2723238387, 1571005438, 2151694528, 93294474, 1066570413, 563977660, 1882732616, 4059428100,
    1673313503, 2008463041, 2950355573, 1109467491, 537923632, 3858759450, 4260623118, 3218264685, 2177748300, 403442708,
    638784309, 3287084079, 3193921505, 899127202, 2286175436, 773265209, 2479146071, 1437050866, 4236148354, 2050833735,
    3362022572, 3126681063, 840505643, 3866325909, 3227541664, 427917720, 2655997905, 2749160575, 1143087718, 1412049534,
    999329963, 193497219, 2353415882, 3354324521, 1807268051, 672404540, 2816401017, 3160301282, 369822493, 2916866934,
    3688947771, 1681011286, 1949973070, 336202270, 2454276571, 201721354, 1210328172, 3093060836, 2680341085, 3184776046,
    1135389935, 3294782118, 965841320, 831886756, 3554993207, 4068047243, 3588745010, 2345191491, 1849112409, 3664604599,
    26054028, 2983581028, 2622377682, 1235855840, 3630984372, 2891339514, 4092916743, 3488279077, 3395642799, 4101667470,
    1202630377, 268961816, 1874508501, 4034427016, 1243948399, 1546530418, 941366308, 1470539505, 1941222599, 2546386513,
    3421038627, 2715671932, 3899946140, 1042226977, 2521517021, 1639824860, 227249030, 260737669, 3765465232, 2084453954,
    1907733956, 3429263018, 2420656344, 100860677, 4160157185, 470683154, 3261161891, 1781871967, 2924959737, 1773779408,
    394692241, 2579611992, 974986535, 664706745, 3655459128, 3958962195, 731420851, 571543859, 3530123707, 2849626480,
    126783113, 865375399, 765172662, 1008606754, 361203602, 3387549984, 2278477385, 2857719295, 1344809080, 2782912378,
    59542671, 1503764984, 160008576, 437062935, 1707065306, 3622233649, 2218934982, 3496503480, 2185314755, 697932208,
    1512910199, 504303377, 2075177163, 2824099068, 1841019862, 739644986 ];

const T_1: [u32; 256] = [2781242211, 2230877308, 2582542199, 2381740923, 234877682, 3184946027, 2984144751, 1418839493, 1348481072, 50462977,
    2848876391, 2102799147, 434634494, 1656084439, 3863849899, 2599188086, 1167051466, 2636087938, 1082771913, 2281340285,
    368048890, 3954334041, 3381544775, 201060592, 3963727277, 1739838676, 4250903202, 3930435503, 3206782108, 4149453988,
    2531553906, 1536934080, 3262494647, 484572669, 2923271059, 1783375398, 1517041206, 1098792767, 49674231, 1334037708,
    1550332980, 4098991525, 886171109, 150598129, 2481090929, 1940642008, 1398944049, 1059722517, 201851908, 1385547719,
    1699095331, 1587397571, 674240536, 2704774806, 252314885, 3039795866, 151914247, 908333586, 2602270848, 1038082786,
    651029483, 1766729511, 3447698098, 2682942837, 454166793, 2652734339, 1951935532, 775166490, 758520603, 3000790638,
    4004797018, 4217086112, 4137964114, 1299594043, 1639438038, 3464344499, 2068982057, 1054729187, 1901997871, 2534638724,
    4121318227, 1757008337, 0, 750906861, 1614815264, 535035132, 3363418545, 3988151131, 3201591914, 1183697867,
    3647454910, 1265776953, 3734260298, 3566750796, 3903871064, 1250283471, 1807470800, 717615087, 3847203498, 384695291,
    3313910595, 3617213773, 1432761139, 2484176261, 3481945413, 283769337, 100925954, 2180939647, 4037038160, 1148730428,
    3123027871, 3813386408, 4087501137, 4267549603, 3229630528, 2315620239, 2906624658, 3156319645, 1215313976, 82966005,
    3747855548, 3245848246, 1974459098, 1665278241, 807407632, 451280895, 251524083, 1841287890, 1283575245, 337120268,
    891687699, 801369324, 3787349855, 2721421207, 3431482436, 959321879, 1469301956, 4065699751, 2197585534, 1199193405,
    2898814052, 3887750493, 724703513, 2514908019, 2696962144, 2551808385, 3516813135, 2141445340, 1715741218, 2119445034,
    2872807568, 2198571144, 3398190662, 700968686, 3547052216, 1009259540, 2041044702, 3803995742, 487983883, 1991105499,
    1004265696, 1449407026, 1316239930, 504629770, 3683797321, 168560134, 1816667172, 3837287516, 1570751170, 1857934291,
    4014189740, 2797888098, 2822345105, 2754712981, 936633572, 2347923833, 852879335, 1133234376, 1500395319, 3084545389,
    2348912013, 1689376213, 3533459022, 3762923945, 3034082412, 4205598294, 133428468, 634383082, 2949277029, 2398386810,
    3913789102, 403703816, 3580869306, 2297460856, 1867130149, 1918643758, 607656988, 4049053350, 3346248884, 1368901318,
    600565992, 2090982877, 2632479860, 557719327, 3717614411, 3697393085, 2249034635, 2232388234, 2430627952, 1115438654,
    3295786421, 2865522278, 3633334344, 84280067, 33027830, 303828494, 2747425121, 1600795957, 4188952407, 3496589753,
    2434238086, 1486471617, 658119965, 3106381470, 953803233, 334231800, 3005978776, 857870609, 3151128937, 1890179545,
    2298973838, 2805175444, 3056442267, 574365214, 2450884487, 550103529, 1233637070, 4289353045, 2018519080, 2057691103,
    2399374476, 4166623649, 2148108681, 387583245, 3664101311, 836232934, 3330556482, 3100665960, 3280093505, 2955516313,
    2002398509, 287182607, 3413881008, 4238890068, 3597515707, 975967766];

const T_2: [u32; 256] = [1671808611, 2089089148, 2006576759, 2072901243, 4061003762, 1807603307, 1873927791, 3310653893, 810573872, 16974337,
    1739181671, 729634347, 4263110654, 3613570519, 2883997099, 1989864566, 3393556426, 2191335298, 3376449993, 2106063485,
    4195741690, 1508618841, 1204391495, 4027317232, 2917941677, 3563566036, 2734514082, 2951366063, 2629772188, 2767672228,
    1922491506, 3227229120, 3082974647, 4246528509, 2477669779, 644500518, 911895606, 1061256767, 4144166391, 3427763148,
    878471220, 2784252325, 3845444069, 4043897329, 1905517169, 3631459288, 827548209, 356461077, 67897348, 3344078279,
    593839651, 3277757891, 405286936, 2527147926, 84871685, 2595565466, 118033927, 305538066, 2157648768, 3795705826,
    3945188843, 661212711, 2999812018, 1973414517, 152769033, 2208177539, 745822252, 439235610, 455947803, 1857215598,
    1525593178, 2700827552, 1391895634, 994932283, 3596728278, 3016654259, 695947817, 3812548067, 795958831, 2224493444,
    1408607827, 3513301457, 0, 3979133421, 543178784, 4229948412, 2982705585, 1542305371, 1790891114, 3410398667,
    3201918910, 961245753, 1256100938, 1289001036, 1491644504, 3477767631, 3496721360, 4012557807, 2867154858, 4212583931,
    1137018435, 1305975373, 861234739, 2241073541, 1171229253, 4178635257, 33948674, 2139225727, 1357946960, 1011120188,
    2679776671, 2833468328, 1374921297, 2751356323, 1086357568, 2408187279, 2460827538, 2646352285, 944271416, 4110742005,
    3168756668, 3066132406, 3665145818, 560153121, 271589392, 4279952895, 4077846003, 3530407890, 3444343245, 202643468,
    322250259, 3962553324, 1608629855, 2543990167, 1154254916, 389623319, 3294073796, 2817676711, 2122513534, 1028094525,
    1689045092, 1575467613, 422261273, 1939203699, 1621147744, 2174228865, 1339137615, 3699352540, 577127458, 712922154,
    2427141008, 2290289544, 1187679302, 3995715566, 3100863416, 339486740, 3732514782, 1591917662, 186455563, 3681988059,
    3762019296, 844522546, 978220090, 169743370, 1239126601, 101321734, 611076132, 1558493276, 3260915650, 3547250131,
    2901361580, 1655096418, 2443721105, 2510565781, 3828863972, 2039214713, 3878868455, 3359869896, 928607799, 1840765549,
    2374762893, 3580146133, 1322425422, 2850048425, 1823791212, 1459268694, 4094161908, 3928346602, 1706019429, 2056189050,
    2934523822, 135794696, 3134549946, 2022240376, 628050469, 779246638, 472135708, 2800834470, 3032970164, 3327236038,
    3894660072, 3715932637, 1956440180, 522272287, 1272813131, 3185336765, 2340818315, 2323976074, 1888542832, 1044544574,
    3049550261, 1722469478, 1222152264, 50660867, 4127324150, 236067854, 1638122081, 895445557, 1475980887, 3117443513,
    2257655686, 3243809217, 489110045, 2662934430, 3778599393, 4162055160, 2561878936, 288563729, 1773916777, 3648039385,
    2391345038, 2493985684, 2612407707, 505560094, 2274497927, 3911240169, 3460925390, 1442818645, 678973480, 3749357023,
    2358182796, 2717407649, 2306869641, 219617805, 3218761151, 3862026214, 1120306242, 1756942440, 1103331905, 2578459033,
    762796589, 252780047, 2966125488, 1425844308, 3151392187, 372911126];

const T_3: [u32; 256] = [1667474886, 2088535288, 2004326894, 2071694838, 4075949567, 1802223062, 1869591006, 3318043793, 808472672, 16843522,
    1734846926, 724270422, 4278065639, 3621216949, 2880169549, 1987484396, 3402253711, 2189597983, 3385409673, 2105378810,
    4210693615, 1499065266, 1195886990, 4042263547, 2913856577, 3570689971, 2728590687, 2947541573, 2627518243, 2762274643,
    1920112356, 3233831835, 3082273397, 4261223649, 2475929149, 640051788, 909531756, 1061110142, 4160160501, 3435941763,
    875846760, 2779116625, 3857003729, 4059105529, 1903268834, 3638064043, 825316194, 353713962, 67374088, 3351728789,
    589522246, 3284360861, 404236336, 2526454071, 84217610, 2593830191, 117901582, 303183396, 2155911963, 3806477791,
    3958056653, 656894286, 2998062463, 1970642922, 151591698, 2206440989, 741110872, 437923380, 454765878, 1852748508,
    1515908788, 2694904667, 1381168804, 993742198, 3604373943, 3014905469, 690584402, 3823320797, 791638366, 2223281939,
    1398011302, 3520161977, 0, 3991743681, 538992704, 4244381667, 2981218425, 1532751286, 1785380564, 3419096717,
    3200178535, 960056178, 1246420628, 1280103576, 1482221744, 3486468741, 3503319995, 4025428677, 2863326543, 4227536621,
    1128514950, 1296947098, 859002214, 2240123921, 1162203018, 4193849577, 33687044, 2139062782, 1347481760, 1010582648,
    2678045221, 2829640523, 1364325282, 2745433693, 1077985408, 2408548869, 2459086143, 2644360225, 943212656, 4126475505,
    3166494563, 3065430391, 3671750063, 555836226, 269496352, 4294908645, 4092792573, 3537006015, 3452783745, 202118168,
    320025894, 3974901699, 1600119230, 2543297077, 1145359496, 387397934, 3301201811, 2812801621, 2122220284, 1027426170,
    1684319432, 1566435258, 421079858, 1936954854, 1616945344, 2172753945, 1330631070, 3705438115, 572679748, 707427924,
    2425400123, 2290647819, 1179044492, 4008585671, 3099120491, 336870440, 3739122087, 1583276732, 185277718, 3688593069,
    3772791771, 842159716, 976899700, 168435220, 1229577106, 101059084, 606366792, 1549591736, 3267517855, 3553849021,
    2897014595, 1650632388, 2442242105, 2509612081, 3840161747, 2038008818, 3890688725, 3368567691, 926374254, 1835907034,
    2374863873, 3587531953, 1313788572, 2846482505, 1819063512, 1448540844, 4109633523, 3941213647, 1701162954, 2054852340,
    2930698567, 134748176, 3132806511, 2021165296, 623210314, 774795868, 471606328, 2795958615, 3031746419, 3334885783,
    3907527627, 3722280097, 1953799400, 522133822, 1263263126, 3183336545, 2341176845, 2324333839, 1886425312, 1044267644,
    3048588401, 1718004428, 1212733584, 50529542, 4143317495, 235803164, 1633788866, 892690282, 1465383342, 3115962473,
    2256965911, 3250673817, 488449850, 2661202215, 3789633753, 4177007595, 2560144171, 286339874, 1768537042, 3654906025,
    2391705863, 2492770099, 2610673197, 505291324, 2273808917, 3924369609, 3469625735, 1431699370, 673740880, 3755965093,
    2358021891, 2711746649, 2307489801, 218961690, 3217021541, 3873845719, 1111672452, 1751693520, 1094828930, 2576986153,
    757954394, 252645662, 2964376443, 1414855848, 3149649517, 370555436];


// generate tables for lookup in aes encrypt loop
#[test]
pub fn create_t_tables() {

    let mut w: u32 = 0;

    let mut res0: [u32; 256] = [0u32; 256];
    let mut res1: [u32; 256] = [0u32; 256];
    let mut res2: [u32; 256] = [0u32; 256];
    let mut res3: [u32; 256] = [0u32; 256];

    for i in 0..256 {
        w = ((MULTIPLY_2[SUBSTITUTION[i] as usize] as u32) << 24)
            | ((SUBSTITUTION[i] as u32) << 16)
            | ((SUBSTITUTION[i] as u32) << 8)
            | ((MULTIPLY_3[SUBSTITUTION[i] as usize]) as u32);

        res0[i] = w;
        res1[i] = w << 24 | w >> 8;
        res2[i] = w << 16 | w >> 16;
        res3[i] = w << 8 | w >> 24;

    }

    // print t0
    let mut c = 1;
    print!("const T_0: [u32; 256] = [");
    for r in res0.iter() {

        if c % 10 == 0 {
            print!("{:?}, \n", r);
        } else {
            print!("{:?}, ", r);
        }

        c += 1;

    } print!("]\n\n");

    // print t1
    c = 1;
    print!("const T_1: [u32; 256] = [");
    for r in res1.iter() {

        if c % 10 == 0 {
            print!("{:?}, \n", r);
        } else {
            print!("{:?}, ", r);
        }

        c += 1;

    } print!("]\n\n");

    // print t2
    c = 1;
    print!("const T_2: [u32; 256] = [");
    for r in res2.iter() {

        if c % 10 == 0 {
            print!("{:?}, \n", r);
        } else {
            print!("{:?}, ", r);
        }

        c += 1;

    } print!("]\n\n");

    // print t3
    c = 1;
    print!("const T_3: [u32; 256] = [");
    for r in res3.iter() {

        if c % 10 == 0 {
            print!("{:?}, \n", r);
        } else {
            print!("{:?}, ", r);
        }

        c += 1;

    } print!("]\n\n");

}

// Function to properly print bytes
fn println_bytes(name_str: &str, bytes: &Vec<u8>) {
    print!("{}", name_str);
    for b in bytes {
        print!("{:02x}", b);
    }
    print!("\n");
}

// Function to handle encryption/decryption command with given parameters
pub fn handle_aes_ctr_command(_command: String,
                              key_size: u16,
                              key_bytes: Vec<u8>,
                              iv_bytes: Vec<u8>,
                              input_file_path: std::path::PathBuf,
                              output_file_path: std::path::PathBuf) {

    // definitions
    let mut end_of_file = false;
    let mut counter: u128 = 0;

    // counter for counter mode
    let mut iv_bytes_array = [0u8; 16];
    iv_bytes_array.clone_from_slice(&iv_bytes[0..16]);
    counter = u128::from_be_bytes(iv_bytes_array);

    // expand keys
    let key_count = if key_size == 128 { 11 } else { 15 };
    let expanded_keys = key_expansion(key_bytes, key_count);

    // input file
    let input_file = File::open(input_file_path).unwrap();
    let mut reader = BufReader::new(input_file); //with_capacity(1048576, input_file); //148576
    let mut read_count= 0;
    let mut data_enc = 0u128;
    let mut buffer: [u8; 16] = [0; 16];

    // output file
    let output_file = File::create(output_file_path).unwrap();
    let mut writer = BufWriter::new(output_file); //with_capacity(1048576,output_file); //148576

    loop {

        // read one block of data
        read_count = reader.read(&mut buffer).unwrap();
        end_of_file = read_count != 16;

        // encrypt stuff
        data_enc = encrypt_aes(counter, &expanded_keys) ^ u128::from_be_bytes(buffer);
        counter += 1;

        // end loop if end of file
        if !end_of_file {
            writer.write(&data_enc.to_be_bytes()).unwrap();
        } else {
            writer.write(&data_enc.to_be_bytes()[0..read_count]).unwrap();
            break;
        }
    }
}

#[test]
fn test_key_expand_256_vector() {

    // example key from FIPS
    let key: [u8; 32] = [0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4];
    let key_vec = key.to_vec();

    let generated_keys = key_expansion(key_vec, 15);

    for i in 0..generated_keys.len() {
        println!("{}", generated_keys[i]);
    }

    // test key length
    assert_eq!(generated_keys.len(), 60);

    // test first few vectors (original key)
    assert_eq!(generated_keys[0], 0x603deb10 as u32);
    assert_eq!(generated_keys[1], 0x15ca71be as u32);
    assert_eq!(generated_keys[2], 0x2b73aef0 as u32);
    assert_eq!(generated_keys[3], 0x857d7781 as u32);

    // test next few vectors (expanded key from example vectors)
    assert_eq!(generated_keys[8], 0x9ba35411 as u32);
    assert_eq!(generated_keys[9], 0x8e6925af as u32);
    assert_eq!(generated_keys[10], 0xa51a8b5f as u32);
    assert_eq!(generated_keys[11], 0x2067fcde as u32);
    assert_eq!(generated_keys[12], 0xa8b09c1a as u32);
    assert_eq!(generated_keys[13], 0x93d194cd as u32);

}

#[test]
fn test_key_expand_128_vector() {

    // example key from FIPS
    let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let key_vec = key.to_vec();

    let generated_keys = key_expansion(key_vec, 11);

    for i in 0..generated_keys.len() {
        println!("{}", generated_keys[i]);
    }

    // test key length
    assert_eq!(generated_keys.len(), 44);

    // test first few vectors (original key)
    assert_eq!(generated_keys[0], 0x2b7e1516 as u32);
    assert_eq!(generated_keys[1], 0x28aed2a6 as u32);
    assert_eq!(generated_keys[2], 0xabf71588 as u32);
    assert_eq!(generated_keys[3], 0x09cf4f3c as u32);

    // test next few vectors (expanded key from example vectors)
    assert_eq!(generated_keys[4], 0xa0fafe17 as u32);
    assert_eq!(generated_keys[5], 0x88542cb1 as u32);
    assert_eq!(generated_keys[6], 0x23a33939 as u32);
    assert_eq!(generated_keys[7], 0x2a6c7605 as u32);
}

fn key_expansion(input_key: Vec<u8>, key_count: usize) -> Vec<u32> {

    let mut return_keys = Vec::new();
    let n = if key_count == 11 {16} else { 32 };

    let mut iteration = 1;
    let mut generated_count = 0;
    let mut temp: [u8; 4];
    let mut holder = 0u8;

    // copy input key to output as first 16 bytes
    for byte in input_key.iter() {
        return_keys.push(*byte);
        generated_count += 1;
    }

    // generate new key bytes
    while generated_count < 16 * key_count {

        // copy last 4 bytes of previous key to temp
        temp = [*return_keys.get(return_keys.len() -4).unwrap(),
                *return_keys.get(return_keys.len() -3).unwrap(),
                *return_keys.get(return_keys.len() -2).unwrap(),
                *return_keys.get(return_keys.len() -1).unwrap()];

        // run the core method if a complete key was generated in last iteration
        if generated_count % n == 0 {
            holder = temp[0];

            // sbox four bytes rotatet left for one bit
            temp[0] = SUBSTITUTION[temp[1] as usize];
            temp[1] = SUBSTITUTION[temp[2] as usize];
            temp[2] = SUBSTITUTION[temp[3] as usize];
            temp[3] = SUBSTITUTION[holder as usize];

            // rcon
            temp[0] ^= RCON[iteration as usize];

            iteration += 1;
        } else if key_count > 11 && generated_count % 16 == 0 {
            temp[0] = SUBSTITUTION[temp[0] as usize];
            temp[1] = SUBSTITUTION[temp[1] as usize];
            temp[2] = SUBSTITUTION[temp[2] as usize];
            temp[3] = SUBSTITUTION[temp[3] as usize];
        }

        // xor 4 new bytes to first 4 bytes of the last generated key
        temp[0] ^= return_keys[return_keys.len() - n];
        temp[1] ^= return_keys[return_keys.len() - n + 1];
        temp[2] ^= return_keys[return_keys.len() - n + 2];
        temp[3] ^= return_keys[return_keys.len() - n + 3];

        // append buffer
        return_keys.push(temp[0]);
        return_keys.push(temp[1]);
        return_keys.push(temp[2]);
        return_keys.push(temp[3]);

        // count up generated byte number
        generated_count += 4;

    }

    // cast to u32 vec
    let mut return_u32 = Vec::new();
    for i in 0..key_count*4 {
        return_u32.push(
            ((return_keys[(i * 4)] as u32) << 24) | ((return_keys[(i*4) + 1] as u32) << 16) | ((return_keys[(i*4) + 2] as u32) << 8) | (return_keys[(i*4) + 3] as u32)
        );
    }

    return_u32

}

fn add_round_key(word: &mut [u8], key: &[u8]) {

    for (w, k) in word.iter_mut().zip(key.iter()) {

        *w ^= *k;
    }

}

fn substitute_bytes(word: &mut [u8]) {

    for w in word.iter_mut() {
        *w = SUBSTITUTION[*w as usize];
    }

}

fn shift_rows(word: &mut [u8]) {

    let mut temp: [u8; 16] = [0; 16];

    for(t, w) in temp.iter_mut().zip(word.iter()) {
        *t = *w;
    }

    word[0] = temp[0];
    word[1] = temp[5];
    word[2] = temp[10];
    word[3] = temp[15];

    word[4] = temp[4];
    word[5] = temp[9];
    word[6] = temp[14];
    word[7] = temp[3];

    word[8] = temp[8];
    word[9] = temp[13];
    word[10] = temp[2];
    word[11] = temp[7];

    word[12] = temp[12];
    word[13] = temp[1];
    word[14] = temp[6];
    word[15] = temp[11];

}

fn mix_columns(word: &mut [u8]) {
    let mut temp = [0; 16];

    temp[0] = MULTIPLY_2[word[0] as usize] ^ MULTIPLY_3[word[1] as usize] ^ word[2] ^ word[3];
    temp[1] = word[0] ^ MULTIPLY_2[word[1] as usize] ^ MULTIPLY_3[word[2] as usize] ^ word[3];
    temp[2] = word[0] ^ word[1] ^ MULTIPLY_2[word[2] as usize] ^ MULTIPLY_3[word[3] as usize];
    temp[3] = MULTIPLY_3[word[0] as usize] ^ word[1] ^ word[2] ^ MULTIPLY_2[word[3] as usize];

    temp[4] = MULTIPLY_2[word[4] as usize] ^ MULTIPLY_3[word[5] as usize] ^ word[6] ^ word[7];
    temp[5] = word[4] ^ MULTIPLY_2[word[5] as usize] ^ MULTIPLY_3[word[6] as usize] ^ word[7];
    temp[6] = word[4] ^ word[5] ^ MULTIPLY_2[word[6] as usize] ^ MULTIPLY_3[word[7] as usize];
    temp[7] = MULTIPLY_3[word[4] as usize] ^ word[5] ^ word[6] ^ MULTIPLY_2[word[7] as usize];

    temp[8] = MULTIPLY_2[word[8] as usize] ^ MULTIPLY_3[word[9] as usize] ^ word[10] ^ word[11];
    temp[9] = word[8] ^ MULTIPLY_2[word[9] as usize] ^ MULTIPLY_3[word[10] as usize] ^ word[11];
    temp[10] = word[8] ^ word[9] ^ MULTIPLY_2[word[10] as usize] ^ MULTIPLY_3[word[11] as usize];
    temp[11] = MULTIPLY_3[word[8] as usize] ^ word[9] ^ word[10] ^ MULTIPLY_2[word[11] as usize];

    temp[12] = MULTIPLY_2[word[12] as usize] ^ MULTIPLY_3[word[13] as usize] ^ word[14] ^ word[15];
    temp[13] = word[12] ^ MULTIPLY_2[word[13] as usize] ^ MULTIPLY_3[word[14] as usize] ^ word[15];
    temp[14] = word[12] ^ word[13] ^ MULTIPLY_2[word[14] as usize] ^ MULTIPLY_3[word[15] as usize];
    temp[15] = MULTIPLY_3[word[12] as usize] ^ word[13] ^ word[14] ^ MULTIPLY_2[word[15] as usize];

    for i in 0..16 {
        word[i] = temp[i];
    }

}

fn encrypt_aes(word_num: u128, keys_vector: &[u32]) -> u128 {

    // init
    let mut round_counter = 0;
    let mut word = word_num.to_be_bytes();

    let mut s0: u32 = u32::from_be_bytes(word[0..4].try_into().unwrap());
    let mut s1: u32 = u32::from_be_bytes(word[4..8].try_into().unwrap());
    let mut s2: u32 = u32::from_be_bytes(word[8..12].try_into().unwrap());
    let mut s3: u32 = u32::from_be_bytes(word[12..16].try_into().unwrap());

    let mut tmp0 = 0;
    let mut tmp1 = 0;
    let mut tmp2 = 0;
    let mut tmp3 = 0;

    s0 ^= keys_vector[round_counter*4];
    s1 ^= keys_vector[round_counter*4+1];
    s2 ^= keys_vector[round_counter*4+2];
    s3 ^= keys_vector[round_counter*4+3];

    round_counter += 1;

    // rounds
    while round_counter < (keys_vector.len()/4)-1 {

        tmp0 = T_0[((s0 >> 24) as u8) as usize]
             ^ T_1[((s1 >> 16) as u8) as usize]
             ^ T_2[((s2 >> 8) as u8) as usize]
             ^ T_3[((s3) as u8) as usize]
             ^ keys_vector[round_counter*4];

        tmp1 = T_0[((s1 >> 24) as u8) as usize]
             ^ T_1[((s2 >> 16) as u8) as usize]
             ^ T_2[((s3 >> 8) as u8) as usize]
             ^ T_3[((s0) as u8) as usize]
             ^ keys_vector[round_counter*4+1];

        tmp2 = T_0[((s2 >> 24) as u8) as usize]
             ^ T_1[((s3 >> 16) as u8) as usize]
             ^ T_2[((s0 >> 8) as u8) as usize]
             ^ T_3[((s1) as u8) as usize]
             ^ keys_vector[round_counter*4+2];

        tmp3 = T_0[((s3 >> 24) as u8) as usize]
             ^ T_1[((s0 >> 16) as u8) as usize]
             ^ T_2[((s1 >> 8) as u8) as usize]
             ^ T_3[((s2) as u8) as usize]
             ^ keys_vector[round_counter*4+3];

        s0 = tmp0;
        s1 = tmp1;
        s2 = tmp2;
        s3 = tmp3;

        round_counter += 1;

    }

    // sbox and shift
    s0 = (SUBSTITUTION[((tmp0>>24) as u8) as usize] as u32) << 24 | (SUBSTITUTION[((tmp1>>16) as u8) as usize] as u32) << 16 | (SUBSTITUTION[((tmp2>>8) as u8) as usize] as u32) << 8 | (SUBSTITUTION[((tmp3) as u8) as usize]) as u32;
    s1 = (SUBSTITUTION[((tmp1>>24) as u8) as usize] as u32) << 24 | (SUBSTITUTION[((tmp2>>16) as u8) as usize] as u32) << 16 | (SUBSTITUTION[((tmp3>>8) as u8) as usize] as u32) << 8 | (SUBSTITUTION[((tmp0) as u8) as usize]) as u32;
    s2 = (SUBSTITUTION[((tmp2>>24) as u8) as usize] as u32) << 24 | (SUBSTITUTION[((tmp3>>16) as u8) as usize] as u32) << 16 | (SUBSTITUTION[((tmp0>>8) as u8) as usize] as u32) << 8 | (SUBSTITUTION[((tmp1) as u8) as usize]) as u32;
    s3 = (SUBSTITUTION[((tmp3>>24) as u8) as usize] as u32) << 24 | (SUBSTITUTION[((tmp0>>16) as u8) as usize] as u32) << 16 | (SUBSTITUTION[((tmp1>>8) as u8) as usize] as u32) << 8 | (SUBSTITUTION[((tmp2) as u8) as usize]) as u32;

    // add round key
    s0 ^= keys_vector[round_counter*4];
    s1 ^= keys_vector[round_counter*4+1];
    s2 ^= keys_vector[round_counter*4+2];
    s3 ^= keys_vector[round_counter*4+3];

    // return encoded data as u128 number
    (s0 as u128) << 96 | (s1 as u128) << 64 | (s2 as u128) << 32 | (s3 as u128)

}
