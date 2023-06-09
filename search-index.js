var searchIndex = JSON.parse('{\
"csv":{"doc":"Crate csv reads and writes comma-separated values (CSV) …","t":[13,13,13,13,13,13,13,3,13,4,3,13,4,3,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,5,5,11,11,11,11,11,11,11,11,12,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,12,12],"n":["BareQuote","Eof","FieldCount","InvalidDelimiter","InvalidDelimiter","Io","Io","ParseError","Quote","ReadError","Reader","TrailingComma","WriteError","Writer","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cause","column","comma","comma","comment","description","equal_partially","equal_partially","err","error","field_pos","fields_per_record","flush","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","input_offset","into","into","into","into","into","lazy_quotes","line","new","new","new_reader","new_writer","provide","provide","provide","read","read_all","source","source","source","start_line","to_string","to_string","to_string","trim_leading_space","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","use_crlf","write","write_all","0","0"],"q":["csv","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","csv::ReadError","csv::WriteError"],"d":["bare <code>&quot;</code> in non-quoted-field","","wrong number of fields","invalid field or comment delimiter","invalid field or comment delimiter","","","A ParseError is returned for parsing errors. Line numbers …","extraneous or missing <code>&quot;</code> in quoted-field","Error cause occurred during parsing records.","A Reader reads records from a CSV-encoded file.","extra delimiter at end of line","Error due to writing record.","A Writer writes records using CSV encoding. As returned by …","","","","","","","","","","","","Column (1-based byte index) where the error occurred","Comma is the field delimiter. It is set to comma (‘,’) …","Field delimiter (set to ‘,’ by NewWriter)","","","check if ReadError equals for all variants except <code>Io</code>, and …","check if all fields of ParseError equals except <code>err</code>, and …","The actual error","Reports any error that has occurred during a previous write…","Returns the line and column corresponding to the start of …","","Writes any buffered data to the underlying io::Write. To …","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","","Returns the input stream byte offset of the current reader …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Line where the error occurred","Returns a new Reader that reads from r.","Returns a new Writer that writes to w.","Returns a new Reader that reads from r.","Returns a new Writer that writes to w.","","","","","Reads all the remaining records from <code>r</code>. Each record is a …","","","","Line where the record starts","","","","","","","","","","","","","","","","","","","","True to use \\\\r\\\\n as the line terminator","Writes a single CSV record to w along with any necessary …","Writes multiple CSV records to <code>self</code> using write and then …","",""],"i":[5,5,5,5,14,5,14,0,5,0,0,5,0,0,9,7,5,1,14,9,7,5,1,14,1,1,9,7,9,1,5,1,1,7,9,9,7,5,5,1,1,14,14,9,7,5,5,1,1,14,14,9,9,7,5,1,14,9,1,9,7,0,0,5,1,14,9,9,5,1,14,1,5,1,14,9,9,7,5,1,14,9,7,5,1,14,9,7,5,1,14,7,7,7,20,21],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,[[3,[2]]]],0,0,0,0,[1,4],[[5,5],6],[[1,1],6],0,[7,[[3,[8]]]],[[9,10]],0,[7,11],[[5,12],13],[[5,12],13],[[1,12],13],[[1,12],13],[[14,12],13],[[14,12],13],[[]],[[]],[8,5],[[]],[[]],[5,1],[[]],[8,14],[9,10],[[]],[[]],[[]],[[]],[[]],0,0,[[],9],[[],7],[[],9],[[],7],[15],[15],[15],[9,[[16,[1]]]],[9,[[16,[[18,[[18,[17]]]],1]]]],[5,[[3,[2]]]],[1,[[3,[2]]]],[14,[[3,[2]]]],0,[[],17],[[],17],[[],17],0,[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],19],[[],19],[[],19],[[],19],[[],19],0,[7,[[16,[14]]]],[7,[[16,[14]]]],0,0],"p":[[3,"ParseError"],[8,"Error"],[4,"Option"],[15,"str"],[4,"ReadError"],[15,"bool"],[3,"Writer"],[3,"Error"],[3,"Reader"],[15,"usize"],[6,"Result"],[3,"Formatter"],[6,"Result"],[4,"WriteError"],[3,"Demand"],[4,"Result"],[3,"String"],[3,"Vec"],[3,"TypeId"],[13,"Io"],[13,"Io"]]},\
"encoding":{"doc":"encoding defines interfaces shared by other modules that …","t":[13,4,13,13,13,0,0,0,0,11,11,2,11,11,11,0,11,11,11,11,11,11,11,12,12,12,12,3,3,11,11,11,11,5,5,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,3,3,3,3,3,17,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,3,17,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,8,3,17,17,17,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,10,11,11,5,5,5,5,11,11,11,11,11,11,11,11,10,11,11,10,11,11,10,11,11,5,5,3,11,11,5,5,5,5,5,5,5,0,11,11,11,11,11,11,11,11,13,4,13,11,11,11,11,11,11,11,11,12,12,12],"n":["CorruptInputError","Error","IO","Overflow","Unknown","ascii85","base32","base64","binary","borrow","borrow_mut","csv","fmt","fmt","from","hex","into","into","provide","to_string","try_from","try_into","type_id","0","0","1","1","Decoder","Encoder","borrow","borrow","borrow_mut","borrow_mut","decode","encode","flush","from","from","into","into","max_encoded_len","new","new","read","try_from","try_from","try_into","try_into","type_id","type_id","write","Decoder","Encoder","Encoding","HEX_ENCODING","STD_ENCODING","STD_PADDING","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","decode","decode_string","decoded_len","deref","deref","encode","encode_to_string","encoded_len","flush","from","from","from","from","from","into","into","into","into","into","new","new","new","read","strip_newlines","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","with_padding","write","Encoding","RAW_STD_ENCODING","RAW_URL_ENCODING","STD_ENCODING","STD_PADDING","URL_ENCODING","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","decode","decode_string","decoded_len","deref","deref","deref","deref","encode","encode_to_string","encoded_len","from","from","from","from","from","into","into","into","into","into","is_pad_char","new","strict","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","with_padding","without_padding","BigEndian","ByteOrder","LittleEndian","MAX_VARINT_LEN16","MAX_VARINT_LEN32","MAX_VARINT_LEN64","borrow","borrow","borrow_mut","borrow_mut","fmt","fmt","from","from","into","into","put_uint16","put_uint16","put_uint16","put_uint32","put_uint32","put_uint32","put_uint64","put_uint64","put_uint64","put_uvarint","put_varint","read_uvarint","read_varint","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","uint16","uint16","uint16","uint32","uint32","uint32","uint64","uint64","uint64","uvariant","variant","Dumper","borrow","borrow_mut","decode","decode_string","decoded_len","dump","encode","encode_to_string","encoded_len","errors","flush","from","into","new","try_from","try_into","type_id","write","ErrLength","Error","InvalidByteError","borrow","borrow_mut","fmt","from","into","try_from","try_into","type_id","0","0","1"],"q":["encoding","","","","","","","","","","","","","","","","","","","","","","","encoding::Error","","","","encoding::ascii85","","","","","","","","","","","","","","","","","","","","","","","","encoding::base32","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","encoding::base64","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","encoding::binary","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","encoding::hex","","","","","","","","","","","","","","","","","","","encoding::hex::errors","","","","","","","","","","","encoding::hex::errors::Error","",""],"d":["","","","","","Module ascii85 implements the ascii85 data encoding as …","Module base32 implements base32 encoding as specified by …","Module base64 implements base64 encoding as specified by …","Module binary implements simple translation between …","","","","","","Returns the argument unchanged.","Module hex implements hexadecimal encoding and decoding.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","Decodes <code>src</code> into <code>dst</code>, returning both the number of bytes …","Encodes <code>src</code> into at most max_encoded_len(src.len()) bytes …","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns the maximum length of an encoding of <code>n</code> source …","Constructs a new ascii85 stream decoder.","Returns a new ascii85 stream encoder. Data written to the …","","","","","","","","","","","An <code>Encoding</code> is a radix 32 encoding/decoding scheme, …","The “Extended Hex Alphabet” defined in RFC 4648. It is …","StdEncoding is the standard base32 encoding, as defined in …","Standard padding character","","","","","","","","","","","","","Decodes <code>src</code> using the encoding <code>enc</code>. It writes at most …","Returns the bytes represented by the base32 string <code>s</code>.","Returns the maximum length in bytes of the decoded data …","","","Encodes <code>src</code> using the encoding <code>enc</code>, writing …","Returns the base32 encoding of <code>src</code>.","Returns the length in bytes of the base32 encoding of an …","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Constructs a new base32 stream decoder.","Returns a new base32 stream encoder. Data written to the …","Returns a new <code>Encoding</code> defined by the given alphabet, …","","","","","","","","","","","","","","","","","","","Creates a new encoding identical to <code>self</code> except with a …","","An Encoding is a radix 64 encoding/decoding scheme, …","RAW_STD_ENCODING is the standard raw, unpadded base64 …","RAW_URL_ENCODING is the unpadded alternate base64 encoding …","STD_ENCODING is the standard base64 encoding, as defined in","Standard padding character","URL_ENCODING is the alternate base64 encoding defined in …","","","","","","","","","","","","","Decodes <code>src</code> using the encoding <code>self</code>. It writes at most …","Returns the bytes represented by the base64 string <code>s</code>.","Returns the maximum length in bytes of the decoded data …","","","","","Encodes <code>src</code> using the encoding <code>self</code>, writing …","Returns the base64 encoding of <code>src</code>.","Returns the length in bytes of the base64 encoding of an …","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Returns a new padded Encoding defined by the given …","Creates a new encoding identical to enc except with strict …","","","","","","","","","","","","","","","","","Creates a new encoding identical to <code>self</code> except with a …","Creates a new encoding identical to <code>self</code> except without …","<code>BigEndian</code> is the big-endian implementation of ByteOrder.","A ByteOrder specifies how to convert byte slices into 16-, …","<code>LittleEndian</code> is the little-endian implementation of …","<code>MaxVarintLen16</code> is the maximum length of a varint-encoded …","<code>MaxVarintLen32</code> is the maximum length of a varint-encoded …","<code>MaxVarintLen64</code> is the maximum length of a varint-encoded …","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","Encodes a uint64 into <code>buf</code> and returns the number of bytes …","Encodes an int64 into <code>buf</code> and returns the number of bytes …","Reads an encoded unsigned integer from <code>r</code> and returns it as …","Reads an encoded signed integer from <code>r</code> and returns it as …","","","","","","","","","","","","","","","","","","Decodes a uint64 from <code>buf</code> and returns that value and the …","Decodes an int64 from <code>buf</code> and returns that value and the …","","","","Decodes src into decoded_len(src.len()) bytes, returning …","Returns the bytes represented by the hexadecimal string <code>s</code>.","Returns the length of a decoding of <code>x</code> source bytes. …","Returns a string that contains a hex dump of the given …","Encodes <code>src</code> into encoded_len(src.len()) bytes of <code>dst</code>. As a …","Returns the hexadecimal encoding of src.","Returns the length of an encoding of <code>n</code> source bytes. …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","@dev the claim that “If an error is returned then no …","ErrLength reports an attempt to decode an odd-length input …","","InvalidByteError values describe errors resulting from an …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","",""],"i":[1,0,1,1,1,0,0,0,0,1,1,0,1,1,1,0,1,1,1,1,1,1,1,37,38,37,38,0,0,13,11,13,11,0,0,11,13,11,13,11,0,13,11,13,13,11,13,11,13,11,11,0,0,0,0,0,0,21,20,18,19,14,21,20,18,19,14,14,14,14,14,14,18,19,14,14,14,20,21,20,18,19,14,21,20,18,19,14,21,20,14,21,0,14,21,20,18,19,14,21,20,18,19,14,21,20,18,19,14,14,20,0,0,0,0,0,0,24,25,26,27,23,24,25,26,27,23,23,23,23,23,23,24,25,26,27,23,23,23,24,25,26,27,23,24,25,26,27,23,23,23,23,23,24,25,26,27,23,24,25,26,27,23,24,25,26,27,23,23,23,0,0,0,0,0,0,29,30,29,30,29,30,29,30,29,30,39,29,30,39,29,30,39,29,30,0,0,0,0,29,30,29,30,29,30,29,30,39,29,30,39,29,30,39,29,30,0,0,0,36,36,0,0,0,0,0,0,0,0,36,36,36,36,36,36,36,36,35,0,35,35,35,35,35,35,35,35,35,40,41,41],"f":[0,0,0,0,0,0,0,0,0,[[]],[[]],0,[[1,2],3],[[1,2],3],[[]],0,[1,4],[[]],[5],[[],6],[[],7],[[],7],[[],8],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[9,[[7,[1]]]],[[],10],[11,12],[[]],[[]],[[]],[[]],[10,10],[[],13],[[],11],[13,[[12,[10]]]],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],[11,[[12,[10]]]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[14,14],[[]],[14,[[7,[10,1]]]],[[14,15],[[7,[[17,[16]],1]]]],[[14,10],10],[18,14],[19,14],[14],[14,6],[[14,10],10],[20,12],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[14,21],[14,20],[[],14],[21,[[12,[10]]]],[[],10],[[]],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],[[],8],[[],8],[[],8],[[14,[22,[16]]],14],[20,[[12,[10]]]],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[23,23],[[]],[23,[[7,[10,1]]]],[[23,15],[[7,[[17,[16]],1]]]],[[23,10],10],[24,23],[25,23],[26,23],[27,23],[23],[23,6],[[23,10],10],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[23,16],9],[15,23],[23,23],[[]],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],[[],8],[[],8],[[],8],[[23,28],23],[23,23],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[29,2],3],[[30,2],3],[[]],[[]],[[]],[[]],[31],[31],[31],[32],[32],[32],[33],[33],[33],[33,10],[34,10],[[],[[7,[33,1]]]],[[],[[7,[34,1]]]],[[],6],[[],6],[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],[[],31],[[],31],[[],31],[[],32],[[],32],[[],32],[[],33],[[],33],[[],33],[[]],[[]],0,[[]],[[]],[[],[[7,[10,35]]]],[15,[[7,[[17,[16]],35]]]],[10,10],[[],6],[[],10],[[],6],[10,10],0,[36,12],[[]],[[]],[[],36],[[],7],[[],7],[[],8],[36,[[12,[10]]]],0,0,0,[[]],[[]],[[35,2],3],[[]],[[]],[[],7],[[],7],[[],8],0,0,0],"p":[[4,"Error"],[3,"Formatter"],[6,"Result"],[3,"Error"],[3,"Demand"],[3,"String"],[4,"Result"],[3,"TypeId"],[15,"bool"],[15,"usize"],[3,"Encoder"],[6,"Result"],[3,"Decoder"],[3,"Encoding"],[15,"str"],[15,"u8"],[3,"Vec"],[3,"STD_ENCODING"],[3,"HEX_ENCODING"],[3,"Encoder"],[3,"Decoder"],[4,"Option"],[3,"Encoding"],[3,"RAW_STD_ENCODING"],[3,"RAW_URL_ENCODING"],[3,"STD_ENCODING"],[3,"URL_ENCODING"],[15,"char"],[3,"BigEndian"],[3,"LittleEndian"],[15,"u16"],[15,"u32"],[15,"u64"],[15,"i64"],[4,"Error"],[3,"Dumper"],[13,"CorruptInputError"],[13,"IO"],[8,"ByteOrder"],[13,"ErrLength"],[13,"InvalidByteError"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
