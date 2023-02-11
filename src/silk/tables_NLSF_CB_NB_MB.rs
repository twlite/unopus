use ::libc;

use crate::silk::structs::silk_NLSF_CB_struct;

#[c2rust::src_loc = "34:25"]
static mut silk_NLSF_CB1_NB_MB_Q8: [u8; 320] = [
    12 as libc::c_int as u8,
    35 as libc::c_int as u8,
    60 as libc::c_int as u8,
    83 as libc::c_int as u8,
    108 as libc::c_int as u8,
    132 as libc::c_int as u8,
    157 as libc::c_int as u8,
    180 as libc::c_int as u8,
    206 as libc::c_int as u8,
    228 as libc::c_int as u8,
    15 as libc::c_int as u8,
    32 as libc::c_int as u8,
    55 as libc::c_int as u8,
    77 as libc::c_int as u8,
    101 as libc::c_int as u8,
    125 as libc::c_int as u8,
    151 as libc::c_int as u8,
    175 as libc::c_int as u8,
    201 as libc::c_int as u8,
    225 as libc::c_int as u8,
    19 as libc::c_int as u8,
    42 as libc::c_int as u8,
    66 as libc::c_int as u8,
    89 as libc::c_int as u8,
    114 as libc::c_int as u8,
    137 as libc::c_int as u8,
    162 as libc::c_int as u8,
    184 as libc::c_int as u8,
    209 as libc::c_int as u8,
    230 as libc::c_int as u8,
    12 as libc::c_int as u8,
    25 as libc::c_int as u8,
    50 as libc::c_int as u8,
    72 as libc::c_int as u8,
    97 as libc::c_int as u8,
    120 as libc::c_int as u8,
    147 as libc::c_int as u8,
    172 as libc::c_int as u8,
    200 as libc::c_int as u8,
    223 as libc::c_int as u8,
    26 as libc::c_int as u8,
    44 as libc::c_int as u8,
    69 as libc::c_int as u8,
    90 as libc::c_int as u8,
    114 as libc::c_int as u8,
    135 as libc::c_int as u8,
    159 as libc::c_int as u8,
    180 as libc::c_int as u8,
    205 as libc::c_int as u8,
    225 as libc::c_int as u8,
    13 as libc::c_int as u8,
    22 as libc::c_int as u8,
    53 as libc::c_int as u8,
    80 as libc::c_int as u8,
    106 as libc::c_int as u8,
    130 as libc::c_int as u8,
    156 as libc::c_int as u8,
    180 as libc::c_int as u8,
    205 as libc::c_int as u8,
    228 as libc::c_int as u8,
    15 as libc::c_int as u8,
    25 as libc::c_int as u8,
    44 as libc::c_int as u8,
    64 as libc::c_int as u8,
    90 as libc::c_int as u8,
    115 as libc::c_int as u8,
    142 as libc::c_int as u8,
    168 as libc::c_int as u8,
    196 as libc::c_int as u8,
    222 as libc::c_int as u8,
    19 as libc::c_int as u8,
    24 as libc::c_int as u8,
    62 as libc::c_int as u8,
    82 as libc::c_int as u8,
    100 as libc::c_int as u8,
    120 as libc::c_int as u8,
    145 as libc::c_int as u8,
    168 as libc::c_int as u8,
    190 as libc::c_int as u8,
    214 as libc::c_int as u8,
    22 as libc::c_int as u8,
    31 as libc::c_int as u8,
    50 as libc::c_int as u8,
    79 as libc::c_int as u8,
    103 as libc::c_int as u8,
    120 as libc::c_int as u8,
    151 as libc::c_int as u8,
    170 as libc::c_int as u8,
    203 as libc::c_int as u8,
    227 as libc::c_int as u8,
    21 as libc::c_int as u8,
    29 as libc::c_int as u8,
    45 as libc::c_int as u8,
    65 as libc::c_int as u8,
    106 as libc::c_int as u8,
    124 as libc::c_int as u8,
    150 as libc::c_int as u8,
    171 as libc::c_int as u8,
    196 as libc::c_int as u8,
    224 as libc::c_int as u8,
    30 as libc::c_int as u8,
    49 as libc::c_int as u8,
    75 as libc::c_int as u8,
    97 as libc::c_int as u8,
    121 as libc::c_int as u8,
    142 as libc::c_int as u8,
    165 as libc::c_int as u8,
    186 as libc::c_int as u8,
    209 as libc::c_int as u8,
    229 as libc::c_int as u8,
    19 as libc::c_int as u8,
    25 as libc::c_int as u8,
    52 as libc::c_int as u8,
    70 as libc::c_int as u8,
    93 as libc::c_int as u8,
    116 as libc::c_int as u8,
    143 as libc::c_int as u8,
    166 as libc::c_int as u8,
    192 as libc::c_int as u8,
    219 as libc::c_int as u8,
    26 as libc::c_int as u8,
    34 as libc::c_int as u8,
    62 as libc::c_int as u8,
    75 as libc::c_int as u8,
    97 as libc::c_int as u8,
    118 as libc::c_int as u8,
    145 as libc::c_int as u8,
    167 as libc::c_int as u8,
    194 as libc::c_int as u8,
    217 as libc::c_int as u8,
    25 as libc::c_int as u8,
    33 as libc::c_int as u8,
    56 as libc::c_int as u8,
    70 as libc::c_int as u8,
    91 as libc::c_int as u8,
    113 as libc::c_int as u8,
    143 as libc::c_int as u8,
    165 as libc::c_int as u8,
    196 as libc::c_int as u8,
    223 as libc::c_int as u8,
    21 as libc::c_int as u8,
    34 as libc::c_int as u8,
    51 as libc::c_int as u8,
    72 as libc::c_int as u8,
    97 as libc::c_int as u8,
    117 as libc::c_int as u8,
    145 as libc::c_int as u8,
    171 as libc::c_int as u8,
    196 as libc::c_int as u8,
    222 as libc::c_int as u8,
    20 as libc::c_int as u8,
    29 as libc::c_int as u8,
    50 as libc::c_int as u8,
    67 as libc::c_int as u8,
    90 as libc::c_int as u8,
    117 as libc::c_int as u8,
    144 as libc::c_int as u8,
    168 as libc::c_int as u8,
    197 as libc::c_int as u8,
    221 as libc::c_int as u8,
    22 as libc::c_int as u8,
    31 as libc::c_int as u8,
    48 as libc::c_int as u8,
    66 as libc::c_int as u8,
    95 as libc::c_int as u8,
    117 as libc::c_int as u8,
    146 as libc::c_int as u8,
    168 as libc::c_int as u8,
    196 as libc::c_int as u8,
    222 as libc::c_int as u8,
    24 as libc::c_int as u8,
    33 as libc::c_int as u8,
    51 as libc::c_int as u8,
    77 as libc::c_int as u8,
    116 as libc::c_int as u8,
    134 as libc::c_int as u8,
    158 as libc::c_int as u8,
    180 as libc::c_int as u8,
    200 as libc::c_int as u8,
    224 as libc::c_int as u8,
    21 as libc::c_int as u8,
    28 as libc::c_int as u8,
    70 as libc::c_int as u8,
    87 as libc::c_int as u8,
    106 as libc::c_int as u8,
    124 as libc::c_int as u8,
    149 as libc::c_int as u8,
    170 as libc::c_int as u8,
    194 as libc::c_int as u8,
    217 as libc::c_int as u8,
    26 as libc::c_int as u8,
    33 as libc::c_int as u8,
    53 as libc::c_int as u8,
    64 as libc::c_int as u8,
    83 as libc::c_int as u8,
    117 as libc::c_int as u8,
    152 as libc::c_int as u8,
    173 as libc::c_int as u8,
    204 as libc::c_int as u8,
    225 as libc::c_int as u8,
    27 as libc::c_int as u8,
    34 as libc::c_int as u8,
    65 as libc::c_int as u8,
    95 as libc::c_int as u8,
    108 as libc::c_int as u8,
    129 as libc::c_int as u8,
    155 as libc::c_int as u8,
    174 as libc::c_int as u8,
    210 as libc::c_int as u8,
    225 as libc::c_int as u8,
    20 as libc::c_int as u8,
    26 as libc::c_int as u8,
    72 as libc::c_int as u8,
    99 as libc::c_int as u8,
    113 as libc::c_int as u8,
    131 as libc::c_int as u8,
    154 as libc::c_int as u8,
    176 as libc::c_int as u8,
    200 as libc::c_int as u8,
    219 as libc::c_int as u8,
    34 as libc::c_int as u8,
    43 as libc::c_int as u8,
    61 as libc::c_int as u8,
    78 as libc::c_int as u8,
    93 as libc::c_int as u8,
    114 as libc::c_int as u8,
    155 as libc::c_int as u8,
    177 as libc::c_int as u8,
    205 as libc::c_int as u8,
    229 as libc::c_int as u8,
    23 as libc::c_int as u8,
    29 as libc::c_int as u8,
    54 as libc::c_int as u8,
    97 as libc::c_int as u8,
    124 as libc::c_int as u8,
    138 as libc::c_int as u8,
    163 as libc::c_int as u8,
    179 as libc::c_int as u8,
    209 as libc::c_int as u8,
    229 as libc::c_int as u8,
    30 as libc::c_int as u8,
    38 as libc::c_int as u8,
    56 as libc::c_int as u8,
    89 as libc::c_int as u8,
    118 as libc::c_int as u8,
    129 as libc::c_int as u8,
    158 as libc::c_int as u8,
    178 as libc::c_int as u8,
    200 as libc::c_int as u8,
    231 as libc::c_int as u8,
    21 as libc::c_int as u8,
    29 as libc::c_int as u8,
    49 as libc::c_int as u8,
    63 as libc::c_int as u8,
    85 as libc::c_int as u8,
    111 as libc::c_int as u8,
    142 as libc::c_int as u8,
    163 as libc::c_int as u8,
    193 as libc::c_int as u8,
    222 as libc::c_int as u8,
    27 as libc::c_int as u8,
    48 as libc::c_int as u8,
    77 as libc::c_int as u8,
    103 as libc::c_int as u8,
    133 as libc::c_int as u8,
    158 as libc::c_int as u8,
    179 as libc::c_int as u8,
    196 as libc::c_int as u8,
    215 as libc::c_int as u8,
    232 as libc::c_int as u8,
    29 as libc::c_int as u8,
    47 as libc::c_int as u8,
    74 as libc::c_int as u8,
    99 as libc::c_int as u8,
    124 as libc::c_int as u8,
    151 as libc::c_int as u8,
    176 as libc::c_int as u8,
    198 as libc::c_int as u8,
    220 as libc::c_int as u8,
    237 as libc::c_int as u8,
    33 as libc::c_int as u8,
    42 as libc::c_int as u8,
    61 as libc::c_int as u8,
    76 as libc::c_int as u8,
    93 as libc::c_int as u8,
    121 as libc::c_int as u8,
    155 as libc::c_int as u8,
    174 as libc::c_int as u8,
    207 as libc::c_int as u8,
    225 as libc::c_int as u8,
    29 as libc::c_int as u8,
    53 as libc::c_int as u8,
    87 as libc::c_int as u8,
    112 as libc::c_int as u8,
    136 as libc::c_int as u8,
    154 as libc::c_int as u8,
    170 as libc::c_int as u8,
    188 as libc::c_int as u8,
    208 as libc::c_int as u8,
    227 as libc::c_int as u8,
    24 as libc::c_int as u8,
    30 as libc::c_int as u8,
    52 as libc::c_int as u8,
    84 as libc::c_int as u8,
    131 as libc::c_int as u8,
    150 as libc::c_int as u8,
    166 as libc::c_int as u8,
    186 as libc::c_int as u8,
    203 as libc::c_int as u8,
    229 as libc::c_int as u8,
    37 as libc::c_int as u8,
    48 as libc::c_int as u8,
    64 as libc::c_int as u8,
    84 as libc::c_int as u8,
    104 as libc::c_int as u8,
    118 as libc::c_int as u8,
    156 as libc::c_int as u8,
    177 as libc::c_int as u8,
    201 as libc::c_int as u8,
    230 as libc::c_int as u8,
];
#[c2rust::src_loc = "77:25"]
static mut silk_NLSF_CB1_Wght_Q9: [i16; 320] = [
    2897 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2888 as libc::c_int as i16,
    2580 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2487 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    3216 as libc::c_int as i16,
    2766 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2460 as libc::c_int as i16,
    2474 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2407 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    3479 as libc::c_int as i16,
    3056 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    3282 as libc::c_int as i16,
    3141 as libc::c_int as i16,
    2580 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2114 as libc::c_int as i16,
    4096 as libc::c_int as i16,
    3845 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2620 as libc::c_int as i16,
    2620 as libc::c_int as i16,
    2407 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2074 as libc::c_int as i16,
    3178 as libc::c_int as i16,
    3244 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2553 as libc::c_int as i16,
    2434 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2167 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    3338 as libc::c_int as i16,
    3488 as libc::c_int as i16,
    2726 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    2460 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2101 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2460 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    3779 as libc::c_int as i16,
    3629 as libc::c_int as i16,
    2434 as libc::c_int as i16,
    2527 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2048 as libc::c_int as i16,
    3254 as libc::c_int as i16,
    3225 as libc::c_int as i16,
    2713 as libc::c_int as i16,
    2846 as libc::c_int as i16,
    2447 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    3263 as libc::c_int as i16,
    3300 as libc::c_int as i16,
    2753 as libc::c_int as i16,
    2806 as libc::c_int as i16,
    2447 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    2101 as libc::c_int as i16,
    2873 as libc::c_int as i16,
    2981 as libc::c_int as i16,
    2633 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2407 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2114 as libc::c_int as i16,
    3225 as libc::c_int as i16,
    3197 as libc::c_int as i16,
    2633 as libc::c_int as i16,
    2580 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2141 as libc::c_int as i16,
    3178 as libc::c_int as i16,
    3310 as libc::c_int as i16,
    2740 as libc::c_int as i16,
    2407 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2194 as libc::c_int as i16,
    2114 as libc::c_int as i16,
    3141 as libc::c_int as i16,
    3272 as libc::c_int as i16,
    2460 as libc::c_int as i16,
    2061 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2500 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2487 as libc::c_int as i16,
    2434 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    3507 as libc::c_int as i16,
    3282 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2700 as libc::c_int as i16,
    2647 as libc::c_int as i16,
    2474 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    3423 as libc::c_int as i16,
    3535 as libc::c_int as i16,
    3038 as libc::c_int as i16,
    3056 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    1950 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    3404 as libc::c_int as i16,
    3366 as libc::c_int as i16,
    2087 as libc::c_int as i16,
    2687 as libc::c_int as i16,
    2873 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2474 as libc::c_int as i16,
    2540 as libc::c_int as i16,
    3760 as libc::c_int as i16,
    3488 as libc::c_int as i16,
    1950 as libc::c_int as i16,
    2660 as libc::c_int as i16,
    2897 as libc::c_int as i16,
    2527 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2460 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    3028 as libc::c_int as i16,
    3272 as libc::c_int as i16,
    2740 as libc::c_int as i16,
    2888 as libc::c_int as i16,
    2740 as libc::c_int as i16,
    2154 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2234 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    3695 as libc::c_int as i16,
    3657 as libc::c_int as i16,
    2025 as libc::c_int as i16,
    1969 as libc::c_int as i16,
    2660 as libc::c_int as i16,
    2700 as libc::c_int as i16,
    2580 as libc::c_int as i16,
    2500 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    3207 as libc::c_int as i16,
    3413 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2074 as libc::c_int as i16,
    2888 as libc::c_int as i16,
    2888 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2487 as libc::c_int as i16,
    2247 as libc::c_int as i16,
    2167 as libc::c_int as i16,
    3338 as libc::c_int as i16,
    3366 as libc::c_int as i16,
    2846 as libc::c_int as i16,
    2780 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2154 as libc::c_int as i16,
    2274 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2114 as libc::c_int as i16,
    2061 as libc::c_int as i16,
    2327 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    2167 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    2633 as libc::c_int as i16,
    2700 as libc::c_int as i16,
    2700 as libc::c_int as i16,
    2553 as libc::c_int as i16,
    2407 as libc::c_int as i16,
    2434 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2261 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2340 as libc::c_int as i16,
    2420 as libc::c_int as i16,
    2607 as libc::c_int as i16,
    2700 as libc::c_int as i16,
    3038 as libc::c_int as i16,
    3244 as libc::c_int as i16,
    2806 as libc::c_int as i16,
    2888 as libc::c_int as i16,
    2474 as libc::c_int as i16,
    2074 as libc::c_int as i16,
    2300 as libc::c_int as i16,
    2314 as libc::c_int as i16,
    2354 as libc::c_int as i16,
    2380 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2154 as libc::c_int as i16,
    2127 as libc::c_int as i16,
    2287 as libc::c_int as i16,
    2500 as libc::c_int as i16,
    2793 as libc::c_int as i16,
    2793 as libc::c_int as i16,
    2620 as libc::c_int as i16,
    2580 as libc::c_int as i16,
    2367 as libc::c_int as i16,
    3676 as libc::c_int as i16,
    3713 as libc::c_int as i16,
    2234 as libc::c_int as i16,
    1838 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    2753 as libc::c_int as i16,
    2726 as libc::c_int as i16,
    2673 as libc::c_int as i16,
    2513 as libc::c_int as i16,
    2207 as libc::c_int as i16,
    2793 as libc::c_int as i16,
    3160 as libc::c_int as i16,
    2726 as libc::c_int as i16,
    2553 as libc::c_int as i16,
    2846 as libc::c_int as i16,
    2513 as libc::c_int as i16,
    2181 as libc::c_int as i16,
    2394 as libc::c_int as i16,
    2221 as libc::c_int as i16,
    2181 as libc::c_int as i16,
];
#[c2rust::src_loc = "112:25"]
static mut silk_NLSF_CB1_iCDF_NB_MB: [u8; 64] = [
    212 as libc::c_int as u8,
    178 as libc::c_int as u8,
    148 as libc::c_int as u8,
    129 as libc::c_int as u8,
    108 as libc::c_int as u8,
    96 as libc::c_int as u8,
    85 as libc::c_int as u8,
    82 as libc::c_int as u8,
    79 as libc::c_int as u8,
    77 as libc::c_int as u8,
    61 as libc::c_int as u8,
    59 as libc::c_int as u8,
    57 as libc::c_int as u8,
    56 as libc::c_int as u8,
    51 as libc::c_int as u8,
    49 as libc::c_int as u8,
    48 as libc::c_int as u8,
    45 as libc::c_int as u8,
    42 as libc::c_int as u8,
    41 as libc::c_int as u8,
    40 as libc::c_int as u8,
    38 as libc::c_int as u8,
    36 as libc::c_int as u8,
    34 as libc::c_int as u8,
    31 as libc::c_int as u8,
    30 as libc::c_int as u8,
    21 as libc::c_int as u8,
    12 as libc::c_int as u8,
    10 as libc::c_int as u8,
    3 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    245 as libc::c_int as u8,
    244 as libc::c_int as u8,
    236 as libc::c_int as u8,
    233 as libc::c_int as u8,
    225 as libc::c_int as u8,
    217 as libc::c_int as u8,
    203 as libc::c_int as u8,
    190 as libc::c_int as u8,
    176 as libc::c_int as u8,
    175 as libc::c_int as u8,
    161 as libc::c_int as u8,
    149 as libc::c_int as u8,
    136 as libc::c_int as u8,
    125 as libc::c_int as u8,
    114 as libc::c_int as u8,
    102 as libc::c_int as u8,
    91 as libc::c_int as u8,
    81 as libc::c_int as u8,
    71 as libc::c_int as u8,
    60 as libc::c_int as u8,
    52 as libc::c_int as u8,
    43 as libc::c_int as u8,
    35 as libc::c_int as u8,
    28 as libc::c_int as u8,
    20 as libc::c_int as u8,
    19 as libc::c_int as u8,
    18 as libc::c_int as u8,
    12 as libc::c_int as u8,
    11 as libc::c_int as u8,
    5 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "123:25"]
static mut silk_NLSF_CB2_SELECT_NB_MB: [u8; 160] = [
    16 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    0 as libc::c_int as u8,
    99 as libc::c_int as u8,
    66 as libc::c_int as u8,
    36 as libc::c_int as u8,
    36 as libc::c_int as u8,
    34 as libc::c_int as u8,
    36 as libc::c_int as u8,
    34 as libc::c_int as u8,
    34 as libc::c_int as u8,
    34 as libc::c_int as u8,
    34 as libc::c_int as u8,
    83 as libc::c_int as u8,
    69 as libc::c_int as u8,
    36 as libc::c_int as u8,
    52 as libc::c_int as u8,
    34 as libc::c_int as u8,
    116 as libc::c_int as u8,
    102 as libc::c_int as u8,
    70 as libc::c_int as u8,
    68 as libc::c_int as u8,
    68 as libc::c_int as u8,
    176 as libc::c_int as u8,
    102 as libc::c_int as u8,
    68 as libc::c_int as u8,
    68 as libc::c_int as u8,
    34 as libc::c_int as u8,
    65 as libc::c_int as u8,
    85 as libc::c_int as u8,
    68 as libc::c_int as u8,
    84 as libc::c_int as u8,
    36 as libc::c_int as u8,
    116 as libc::c_int as u8,
    141 as libc::c_int as u8,
    152 as libc::c_int as u8,
    139 as libc::c_int as u8,
    170 as libc::c_int as u8,
    132 as libc::c_int as u8,
    187 as libc::c_int as u8,
    184 as libc::c_int as u8,
    216 as libc::c_int as u8,
    137 as libc::c_int as u8,
    132 as libc::c_int as u8,
    249 as libc::c_int as u8,
    168 as libc::c_int as u8,
    185 as libc::c_int as u8,
    139 as libc::c_int as u8,
    104 as libc::c_int as u8,
    102 as libc::c_int as u8,
    100 as libc::c_int as u8,
    68 as libc::c_int as u8,
    68 as libc::c_int as u8,
    178 as libc::c_int as u8,
    218 as libc::c_int as u8,
    185 as libc::c_int as u8,
    185 as libc::c_int as u8,
    170 as libc::c_int as u8,
    244 as libc::c_int as u8,
    216 as libc::c_int as u8,
    187 as libc::c_int as u8,
    187 as libc::c_int as u8,
    170 as libc::c_int as u8,
    244 as libc::c_int as u8,
    187 as libc::c_int as u8,
    187 as libc::c_int as u8,
    219 as libc::c_int as u8,
    138 as libc::c_int as u8,
    103 as libc::c_int as u8,
    155 as libc::c_int as u8,
    184 as libc::c_int as u8,
    185 as libc::c_int as u8,
    137 as libc::c_int as u8,
    116 as libc::c_int as u8,
    183 as libc::c_int as u8,
    155 as libc::c_int as u8,
    152 as libc::c_int as u8,
    136 as libc::c_int as u8,
    132 as libc::c_int as u8,
    217 as libc::c_int as u8,
    184 as libc::c_int as u8,
    184 as libc::c_int as u8,
    170 as libc::c_int as u8,
    164 as libc::c_int as u8,
    217 as libc::c_int as u8,
    171 as libc::c_int as u8,
    155 as libc::c_int as u8,
    139 as libc::c_int as u8,
    244 as libc::c_int as u8,
    169 as libc::c_int as u8,
    184 as libc::c_int as u8,
    185 as libc::c_int as u8,
    170 as libc::c_int as u8,
    164 as libc::c_int as u8,
    216 as libc::c_int as u8,
    223 as libc::c_int as u8,
    218 as libc::c_int as u8,
    138 as libc::c_int as u8,
    214 as libc::c_int as u8,
    143 as libc::c_int as u8,
    188 as libc::c_int as u8,
    218 as libc::c_int as u8,
    168 as libc::c_int as u8,
    244 as libc::c_int as u8,
    141 as libc::c_int as u8,
    136 as libc::c_int as u8,
    155 as libc::c_int as u8,
    170 as libc::c_int as u8,
    168 as libc::c_int as u8,
    138 as libc::c_int as u8,
    220 as libc::c_int as u8,
    219 as libc::c_int as u8,
    139 as libc::c_int as u8,
    164 as libc::c_int as u8,
    219 as libc::c_int as u8,
    202 as libc::c_int as u8,
    216 as libc::c_int as u8,
    137 as libc::c_int as u8,
    168 as libc::c_int as u8,
    186 as libc::c_int as u8,
    246 as libc::c_int as u8,
    185 as libc::c_int as u8,
    139 as libc::c_int as u8,
    116 as libc::c_int as u8,
    185 as libc::c_int as u8,
    219 as libc::c_int as u8,
    185 as libc::c_int as u8,
    138 as libc::c_int as u8,
    100 as libc::c_int as u8,
    100 as libc::c_int as u8,
    134 as libc::c_int as u8,
    100 as libc::c_int as u8,
    102 as libc::c_int as u8,
    34 as libc::c_int as u8,
    68 as libc::c_int as u8,
    68 as libc::c_int as u8,
    100 as libc::c_int as u8,
    68 as libc::c_int as u8,
    168 as libc::c_int as u8,
    203 as libc::c_int as u8,
    221 as libc::c_int as u8,
    218 as libc::c_int as u8,
    168 as libc::c_int as u8,
    167 as libc::c_int as u8,
    154 as libc::c_int as u8,
    136 as libc::c_int as u8,
    104 as libc::c_int as u8,
    70 as libc::c_int as u8,
    164 as libc::c_int as u8,
    246 as libc::c_int as u8,
    171 as libc::c_int as u8,
    137 as libc::c_int as u8,
    139 as libc::c_int as u8,
    137 as libc::c_int as u8,
    155 as libc::c_int as u8,
    218 as libc::c_int as u8,
    219 as libc::c_int as u8,
    139 as libc::c_int as u8,
];
#[c2rust::src_loc = "146:25"]
static mut silk_NLSF_CB2_iCDF_NB_MB: [u8; 72] = [
    255 as libc::c_int as u8,
    254 as libc::c_int as u8,
    253 as libc::c_int as u8,
    238 as libc::c_int as u8,
    14 as libc::c_int as u8,
    3 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    254 as libc::c_int as u8,
    252 as libc::c_int as u8,
    218 as libc::c_int as u8,
    35 as libc::c_int as u8,
    3 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    254 as libc::c_int as u8,
    250 as libc::c_int as u8,
    208 as libc::c_int as u8,
    59 as libc::c_int as u8,
    4 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    254 as libc::c_int as u8,
    246 as libc::c_int as u8,
    194 as libc::c_int as u8,
    71 as libc::c_int as u8,
    10 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    252 as libc::c_int as u8,
    236 as libc::c_int as u8,
    183 as libc::c_int as u8,
    82 as libc::c_int as u8,
    8 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    252 as libc::c_int as u8,
    235 as libc::c_int as u8,
    180 as libc::c_int as u8,
    90 as libc::c_int as u8,
    17 as libc::c_int as u8,
    2 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    248 as libc::c_int as u8,
    224 as libc::c_int as u8,
    171 as libc::c_int as u8,
    97 as libc::c_int as u8,
    30 as libc::c_int as u8,
    4 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
    255 as libc::c_int as u8,
    254 as libc::c_int as u8,
    236 as libc::c_int as u8,
    173 as libc::c_int as u8,
    95 as libc::c_int as u8,
    37 as libc::c_int as u8,
    7 as libc::c_int as u8,
    1 as libc::c_int as u8,
    0 as libc::c_int as u8,
];
#[c2rust::src_loc = "158:25"]
static mut silk_NLSF_CB2_BITS_NB_MB_Q5: [u8; 72] = [
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    131 as libc::c_int as u8,
    6 as libc::c_int as u8,
    145 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    236 as libc::c_int as u8,
    93 as libc::c_int as u8,
    15 as libc::c_int as u8,
    96 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    194 as libc::c_int as u8,
    83 as libc::c_int as u8,
    25 as libc::c_int as u8,
    71 as libc::c_int as u8,
    221 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    162 as libc::c_int as u8,
    73 as libc::c_int as u8,
    34 as libc::c_int as u8,
    66 as libc::c_int as u8,
    162 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    210 as libc::c_int as u8,
    126 as libc::c_int as u8,
    73 as libc::c_int as u8,
    43 as libc::c_int as u8,
    57 as libc::c_int as u8,
    173 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    201 as libc::c_int as u8,
    125 as libc::c_int as u8,
    71 as libc::c_int as u8,
    48 as libc::c_int as u8,
    58 as libc::c_int as u8,
    130 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    166 as libc::c_int as u8,
    110 as libc::c_int as u8,
    73 as libc::c_int as u8,
    57 as libc::c_int as u8,
    62 as libc::c_int as u8,
    104 as libc::c_int as u8,
    210 as libc::c_int as u8,
    255 as libc::c_int as u8,
    255 as libc::c_int as u8,
    251 as libc::c_int as u8,
    123 as libc::c_int as u8,
    65 as libc::c_int as u8,
    55 as libc::c_int as u8,
    68 as libc::c_int as u8,
    100 as libc::c_int as u8,
    171 as libc::c_int as u8,
    255 as libc::c_int as u8,
];
#[c2rust::src_loc = "170:25"]
static mut silk_NLSF_PRED_NB_MB_Q8: [u8; 18] = [
    179 as libc::c_int as u8,
    138 as libc::c_int as u8,
    140 as libc::c_int as u8,
    148 as libc::c_int as u8,
    151 as libc::c_int as u8,
    149 as libc::c_int as u8,
    153 as libc::c_int as u8,
    151 as libc::c_int as u8,
    163 as libc::c_int as u8,
    116 as libc::c_int as u8,
    67 as libc::c_int as u8,
    82 as libc::c_int as u8,
    59 as libc::c_int as u8,
    92 as libc::c_int as u8,
    72 as libc::c_int as u8,
    100 as libc::c_int as u8,
    89 as libc::c_int as u8,
    92 as libc::c_int as u8,
];
#[c2rust::src_loc = "176:25"]
static mut silk_NLSF_DELTA_MIN_NB_MB_Q15: [i16; 11] = [
    250 as libc::c_int as i16,
    3 as libc::c_int as i16,
    6 as libc::c_int as i16,
    3 as libc::c_int as i16,
    3 as libc::c_int as i16,
    3 as libc::c_int as i16,
    4 as libc::c_int as i16,
    3 as libc::c_int as i16,
    3 as libc::c_int as i16,
    3 as libc::c_int as i16,
    461 as libc::c_int as i16,
];
#[no_mangle]
#[c2rust::src_loc = "181:27"]
pub static mut silk_NLSF_CB_NB_MB: silk_NLSF_CB_struct = unsafe {
    {
        let init = silk_NLSF_CB_struct {
            nVectors: 32 as libc::c_int as i16,
            order: 10 as libc::c_int as i16,
            quantStepSize_Q16: (0.18f64
                * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as i32 as i16,
            invQuantStepSize_Q6: (1.0f64 / 0.18f64
                * ((1 as libc::c_int as i64) << 6 as libc::c_int) as libc::c_double
                + 0.5f64) as i32 as i16,
            CB1_NLSF_Q8: silk_NLSF_CB1_NB_MB_Q8.as_ptr(),
            CB1_Wght_Q9: silk_NLSF_CB1_Wght_Q9.as_ptr(),
            CB1_iCDF: silk_NLSF_CB1_iCDF_NB_MB.as_ptr(),
            pred_Q8: silk_NLSF_PRED_NB_MB_Q8.as_ptr(),
            ec_sel: silk_NLSF_CB2_SELECT_NB_MB.as_ptr(),
            ec_iCDF: silk_NLSF_CB2_iCDF_NB_MB.as_ptr(),
            ec_Rates_Q5: silk_NLSF_CB2_BITS_NB_MB_Q5.as_ptr(),
            deltaMin_Q15: silk_NLSF_DELTA_MIN_NB_MB_Q15.as_ptr(),
        };
        init
    }
};
