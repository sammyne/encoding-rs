window.SIDEBAR_ITEMS = {"constant":[["MAX_VARINT_LEN16","`MaxVarintLen16` is the maximum length of a varint-encoded 16-bit integer."],["MAX_VARINT_LEN32","`MaxVarintLen32` is the maximum length of a varint-encoded 32-bit integer."],["MAX_VARINT_LEN64","`MaxVarintLen64` is the maximum length of a varint-encoded 64-bit integer."]],"enum":[["Error",""]],"fn":[["put_uvarint","Encodes a uint64 into `buf` and returns the number of bytes written. If the buffer is too small, `put_uvarint` will panic."],["put_varint","Encodes an int64 into `buf` and returns the number of bytes written. If the buffer is too small, `put_varint` will panic."],["read_uvarint","Reads an encoded unsigned integer from `r` and returns it as a uint64."],["read_varint","Reads an encoded signed integer from `r` and returns it as an int64."],["uvariant","Decodes a uint64 from `buf` and returns that value and the number of bytes read (> 0). If an error occurred, the value is 0 and the number of bytes `n` is <= 0 meaning:"],["variant","Decodes an int64 from `buf` and returns that value and the number of bytes read (> 0). If an error occurred, the value is 0 and the number of bytes `n` is <= 0 with the following meaning:"]],"struct":[["BigEndian","`BigEndian` is the big-endian implementation of [ByteOrder]."],["LittleEndian","`LittleEndian` is the little-endian implementation of [ByteOrder]."]],"trait":[["ByteOrder","A ByteOrder specifies how to convert byte slices into 16-, 32-, or 64-bit unsigned integers."]]};