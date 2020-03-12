ucd-generate.exe property-bool ucd-10.0.0 --include XID_Start,XID_Continue,ID_START,ID_Continue,Other_ID_START,Other_ID_Continue --chars > xid.rs
ucd-generate.exe property-bool ucd-10.0.0 --include WHITE_SPACE,PATTERN_WHITE_SPACE --chars > space.rs
ucd-generate.exe property-bool ucd-10.0.0 --include MATH,OTHER_MATH --chars > math.rs
