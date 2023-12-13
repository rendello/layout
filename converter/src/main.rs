
use phf::phf_map;

enum Ident {
    INVALID,
    AI,    U,     UU,    I,     II,    A,     AA,           /* ᐊᐃᐅᐁ */
    PAI,   PU,    PUU,   PI,    PII,   PA,    PAA,   P,     /* ᐸᐱᐳᐯᑉ */
    TAI,   TU,    TUU,   TI,    TII,   TA,    TAA,   T,     /* ᑕᑎᑐᑌᑦ */
    KAI,   KU,    KUU,   KI,    KII,   KA,    KAA,   K,     /* ᑲᑭᑯᑫᒃ */
    HAI,   HU,    HUU,   HI,    HII,   HA,    HAA,   H,     /* ᕹᕵᕷᕴᕻ */
    GAI,   GU,    GUU,   GI,    GII,   GA,    GAA,   G,     /* ᒐᒋᒍᒉᒡ */
    MAI,   MU,    MUU,   MI,    MII,   MA,    MAA,   M,     /* ᒪᒥᒧᒣᒻ */
    NAI,   NU,    NUU,   NI,    NII,   NA,    NAA,   N,     /* ᓇᓂᓄᓀᓐ */
    SAI,   SU,    SUU,   SI,    SII,   SA,    SAA,   S,     /* ᓴᓯᓱᓭᔅ */
           ŠU,    ŠUU,   ŠI,    ŠII,   ŠA,    ŠAA,   Š,     /* 𑪺𑪶𑪸 */
           HU2,   HUU2,  HI2,   HII2,  HA2,   HAA2,  H2,    /* 𑪴𑪰𑪲 */
    LAI,   LU,    LUU,   LI,    LII,   LA,    LAA,   L,     /* ᓚᓕᓗᓓᓪ */
    JAI,   JU,    JUU,   JI,    JII,   JA,    JAA,   J,     /* ᔭᔨᔪᔦᔾ */
    JJAI,  JJU,   JJUU,  JJI,   JJII,  JJA,   JJAA,  JJ,    /* ᑦᔭᑦᔨᑦᔪᑦᔦᑦᔾ */
           ŘU,    ŘUU,   ŘI,    ŘII,   ŘA,    ŘAA,   Ř,     /* ᖬᖨᖪᖮ */
    VAI,   VU,    VUU,   VI,    VII,   VA,    VAA,   V,     /* ᕙᕕᕗᕓᕝ */
    RAI,   RU,    RUU,   RI,    RII,   RA,    RAA,   R,     /* ᕋᕆᕈᕂᕐ */
    QAI,   QU,    QUU,   QI,    QII,   QA,    QAA,   Q,     /* ᖃᕿᖁᙯᖅ */
    QQAI,  QQU,   QQUU,  QQI,   QQII,  QQA,   QQAA,  QQ,    /* ᖅᑲᖅᑭᖅᑯᖅᑫᖅᒃ */
    NGAI,  NGU,   NGUU,  NGI,   NGII,  NGA,   NGAA,  NG,    /* ᖓᖏᖑᙰᖕ */
           NNGU,  NNGUU, NNGI,  NNGII, NNGA,  NNGAA, NNG,   /* ᙵᙱᙳᖖ */
           ŁU,    ŁUU,   ŁI,    ŁII,   ŁA,    ŁAA,   Ł,     /* ᖤᖠᖢᖦ */
                                                     B,     /* ᖯ */
                                                     H3,    /* ᕼ */
                                                     STOP,  /* ᑊ */
}

impl Ident {
    fn syllabics_to_idents(s: &str) -> Vec<Ident> {

    }
}