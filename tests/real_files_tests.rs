// Testing suite for the "real files". Calculate the needed hours and expected output of all these
// files and check!
//
// 200:
//
// >>> (12+5+2+9+14)/60
// 0.7
// >>> (13)/60
// 0.21666666666666667
// >>> (10+60+23+10+12)/60
// 1.9166666666666667
// >>> (37+60+31)/60
// 2.1333333333333333
// >>> (11+26+60+19)/60
// 1.9333333333333333
// >>> (20)/60
// 0.3333333333333333
// >>>
//
// ADMIN:
// General Admin: 0.7
//
// BHL1890:
// Job Admin: 0.22
//
// OZT2122:
// Job Admin: 1.92
// R&D??????: 2.13
//
// ZAP2027:
// Drafting: 1.93
//
// PTD2023
// Job Admin: 0.33
//
// Actual output:
// # 2026/01/16, # #BHL1890, ## Job Admin, 0.69
// # 2026/01/16, # #OZT2122, ## Job Admin, 0.22
// # 2026/01/16, # #OZT2122, ## R&D??????, 1.92
// # 2026/01/16, # #ZAP2027, ## Drafting, 2.13
// # 2026/01/16, # #PTD2023, ## Job Admin, 1.9300001
//
//
// 201
//>>> (19+19+13+2)/60
// 0.8833333333333333
// >>> (120+32)/60
// 2.533333333333333
// >>> (60+8)/60
// 1.1333333333333333
// >>> (60+32+4)/60
// 1.6
// >>> (28+60+12)/60
// 1.6666666666666667
// >>>
//
// Admin:
// General Admin: 0.88
//
// BHL1890:
// Job Admin: 2.53
//
// PTD2023:
// Job Admin: 1.13
//
// ZAP2027:
// R&D: 1.6
//
// DEF1140:
// Job Admin: 1.67
//
// , # #BHL1890, ## Job Admin, 0.89
// , # #PTD2023, ## Job Admin, 2.53
// , # #ZAP2027, ## R&D, 1.13
// , # #DEF1140, ## Job Admin, 1.6

// Wait where is the general admin section??? Everything is offset.
