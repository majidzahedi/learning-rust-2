// Challenge 02: Dissect a Ping, from the wire up to the IP layer
// Run: cargo run --bin challenge_02_packet
//
// Below is ONE real Ethernet frame, exactly as a network card would hand it to the OS.
// It's a `ping 8.8.8.8` sent from a machine on a home network.
// Your job: decode it byte by byte, verify it, and then act like a router.
//
// Uses ONLY Part 1: integers, arrays, functions, loops, casting.
// NEW: bit operations. That's part of the research!
//
// ============================================================================
// STEP 0: RESEARCH FIRST
// ============================================================================
// Answer in a comment block here before coding. Good sources: Wikipedia pages for
// "Ethernet frame", "IPv4" (the header diagram!), "ICMP", and the RFCs themselves:
// RFC 791 (IPv4), RFC 792 (ICMP), RFC 1071 (the Internet checksum). RFCs are readable,
// don't be scared of them.
//
//   Q1.  Layers: what does layer 1, layer 2 and layer 3 each do? Which header in this
//        frame belongs to which layer? (ICMP is a bit special, where does it live?)
//   Q2.  Ethernet II header: which fields, how many bytes each? What is the EtherType
//        for IPv4? For ARP? For IPv6?
//   Q3.  What is "network byte order"? Is it big-endian or little-endian?
//   Q4.  IPv4 header: draw the first 20 bytes. What does IHL count, bytes or something else?
//        Why could an IPv4 header be longer than 20 bytes?
//   Q5.  What are the DF and MF flags? Why is the fragment offset multiplied by 8?
//   Q6.  Protocol numbers: what are 1, 6 and 17?
//   Q7.  The Internet checksum (RFC 1071): how is it computed? What is "ones' complement
//        addition" and what does "fold the carry" mean?
//        How do you VERIFY a checksum without recomputing it separately?
//   Q8.  This frame is 60 bytes, but the IPv4 "total length" says 36. Count the rest.
//        Why is there zero-padding at the end? (Search: "Ethernet minimum frame size")
//   Q9.  ICMP echo: what are type 8 and type 0? What is TTL, and what does a router send
//        back when TTL hits 0? (This is how `traceroute` works!)
//   Q10. RFC 1918: which IPv4 ranges are private?
//
// ============================================================================
// BIT OPERATIONS PRIMER (you'll need all of these)
// ============================================================================
//   a & b    AND: keep only bits set in both.    0x45 & 0x0F == 0x05  (a "mask")
//   a | b    OR:  combine bits.                  0x40 | 0x05 == 0x45
//   a ^ b    XOR: flip bits where b has a 1.     0x45 ^ 0x01 == 0x44
//   !a       NOT: flip ALL bits.                 !0x00u8 == 0xFF  (not `~` like C!)
//   a << n   shift left  (multiply by 2^n)       0x01 << 4 == 0x10
//   a >> n   shift right (divide by 2^n)         0x45 >> 4 == 0x04
//   Print in hex/binary: {:02x}  {:04x}  {:#06x}  {:08b}
//   GOTCHA: `0xAB_u8 << 8` overflows a u8. Cast FIRST: (byte as u16) << 8
//   GOTCHA: shifting a u32 by 32 or more panics in debug builds. Remember this for TODO 8!

const FRAME: [u8; 60] = [
    0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x08, 0x00, 0x27, 0x13, 0x37, 0x42, //
    0x08, 0x00, 0x45, 0x00, 0x00, 0x24, 0x1c, 0x46, 0x40, 0x00, 0x40, 0x01, //
    0x4c, 0xb1, 0xc0, 0xa8, 0x01, 0x2a, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, //
    0x20, 0x0d, 0x13, 0x37, 0x00, 0x01, 0x72, 0x75, 0x73, 0x74, 0x70, 0x69, //
    0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
];

// Arrays of u8 are Copy, so passing FRAME (or a modified copy) into functions just works.
// Slices (&[u8], lesson 08) are the proper tool for this, and you'll rewrite it with them later.

// ============================================================================
// LAYER 2: ETHERNET
// ============================================================================

// TODO 1: `fn read_u16(frame: [u8; 60], offset: usize) -> u16`
//         Read 2 bytes as ONE big-endian number. read_u16(FRAME, 12) == 0x0800
//         Hint: high byte shifted left by 8, OR'd with the low byte. Watch the GOTCHA above.
fn read_u16(frame: [u8; 60], offset: usize) -> u16 {
    ((frame[offset] as u16) << 8) | (frame[offset + 1] as u16)
}

// TODO 2: `fn print_mac(frame: [u8; 60], offset: usize)`
//         Print 6 bytes as a MAC address: 00:1a:2b:3c:4d:5e
//         Hint: {:02x}, and print ':' between bytes but not after the last one.
fn print_mac(frame: [u8; 60], offset: usize) {
    for i in 0..6 {
        print!("{:02x}", frame[offset + i]);
        if i != 5 {
            print!(":")
        }
    }
}

// TODO 3: Print the Ethernet header: destination MAC, source MAC, EtherType.
//         Translate the EtherType to a name ("IPv4", "ARP", "IPv6", "unknown").
//         Research bonus: the lowest bit of the FIRST byte of a MAC address has a special
//         meaning (search: "MAC address I/G bit"). Is the destination unicast or multicast?
//         What is ff:ff:ff:ff:ff:ff?
fn print_ethernet_header(frame: [u8; 60]) {
    println!("destination MAC is: ");
    print_mac(frame, 0);
    println!();
    println!("source MAC is: ");
    print_mac(frame, 6);
    println!();
    let ether_type = read_u16(frame, 12);
    if ether_type == 0x0800 {
        print!("IPv4");
    } else if ether_type == 0x806 {
        print!("ARP");
    } else if ether_type == 0x86DD {
        print!("IPv6");
    } else {
        print!("unkown");
    }
}
// ============================================================================
// LAYER 3: IPv4
// ============================================================================

const IP: usize = 14; // where the IPv4 header starts: right after the 14-byte Ethernet header

// TODO 4: Decode and print EVERY field of the IPv4 header:
//           version and IHL   (one byte, two 4-bit "nibbles": use >> and &)
//           header length in BYTES (from IHL)
//           total length
//           identification   (print in hex)
//           flags: DF and MF as booleans, fragment offset in bytes
//                  (flags + offset share 2 bytes: 3 bits flags, 13 bits offset)
//           TTL
//           protocol, with its name
//           header checksum  (print in hex)
//           source and destination as dotted decimal: 192.168.1.42
//         Expected: version 4, header 20 bytes, total length 36, DF set, TTL 64,
//                   protocol 1 (ICMP), 192.168.1.42 -> 8.8.8.8
fn print_ipv4_header(frame: [u8; 60]) {
    let nibble = 0x0f;
    let version = frame[IP] >> 4 & nibble;
    let ihl = frame[IP] & nibble;

    print!(
        "version {}, header {} bytes, total length",
        version,
        ihl * 4
    )
}

// TODO 5: `fn checksum(frame: [u8; 60], start: usize, end: usize) -> u16`
//         The RFC 1071 Internet checksum over frame[start..end].
//         Hints:
//         - Add up 16-bit big-endian words in a u32 (so carries don't overflow).
//         - "Fold": while the sum has bits above 16, add the high part to the low part.
//         - The result is the ones' complement (NOT) of the folded sum, as u16.
//         - What if the length is ODD? (RFC 1071 says. Not needed for this frame, but be correct.)
//         Then VERIFY the IPv4 header: checksum over the header INCLUDING its checksum
//         field should give a special value. Which one? (Q7.) Print "valid" or "INVALID".

// TODO 6: Break it on purpose. Copy the frame (`let mut bad = FRAME;`), flip ONE bit
//         anywhere in the IPv4 header with ^=, and show that verification now fails.
//         Then flip a bit in the PADDING. Does the IP checksum notice? Why not?

// ============================================================================
// ICMP (carried inside IPv4)
// ============================================================================

// TODO 7: Decode the ICMP message: type (with name), code, checksum, identifier, sequence
//         number, and print the payload bytes as text (`byte as char`).
//         The ICMP message starts after the IP header (use IHL!) and ends at
//         total_length, NOT at the end of the frame. Why? (Q8.)
//         Verify the ICMP checksum with your TODO 5 function. It covers the whole ICMP
//         message (header + payload), unlike the IP checksum.

// ============================================================================
// SUBNETS
// ============================================================================

// TODO 8: `fn ip_to_u32(frame: [u8; 60], offset: usize) -> u32`  (4 bytes, big-endian)
//         `fn in_subnet(ip: u32, network: u32, prefix: u32) -> bool`
//         Hints:
//         - Build a mask from the prefix: /24 -> 0xFFFFFF00. Use !0u32 and <<.
//         - An IP is in the subnet if (ip & mask) == (network & mask).
//         - Edge case: /0 (the whole internet). What does your mask code do? (GOTCHA!)
//           Test it, read the panic, and handle it.
//         Expected for the source 192.168.1.42:
//           in 192.168.1.0/24 -> true      in 192.168.0.0/16 -> true
//           in 192.168.1.0/27 -> false     in 0.0.0.0/0      -> true
//         Then write `fn is_private(ip: u32) -> bool` using Q10. Source private? Destination?

// ============================================================================
// BE A ROUTER
// ============================================================================

// TODO 9: A router forwarding this packet must decrement the TTL, and then the header
//         checksum is wrong, so it must recompute it.
//         - Copy the frame, set TTL to 63
//         - Zero the checksum field, compute a new checksum, write its 2 bytes back
//           (high byte: >> 8, low byte: & 0xFF, cast to u8)
//         - Verify it with TODO 5.  Expected new checksum: 0x4db1
//         - What should the router do if the TTL was 1 when it arrived? (Q9.)
//         Notice: 64 -> 63 changed the checksum by exactly 0x0100. Why?

fn main() {
    // Call and test your functions here as you write them.
    // Tip: print section headers so the output reads like Wireshark:
    //   === Ethernet ===
    //   === IPv4 ===
    //   === ICMP ===

    // tests
    // print_ethernet_header(FRAME);
    print_ipv4_header(FRAME);
}

// ============================================================================
// STRETCH GOALS (pick any)
// ============================================================================
//   A. Build the REPLY that 8.8.8.8 sends back: swap the MACs, swap the IPs, set the
//      ICMP type to 0, recompute both checksums. Verify both with TODO 5.
//      Surprise: the IP checksum is the SAME as the request's. Why?
//      (Hint: what property of addition makes swapping two fields irrelevant?)
//      Expected new ICMP checksum: 0x280d
//
//   B. Incremental update (RFC 1624): recompute the checksum after the TTL change WITHOUT
//      summing the whole header again, using only the old checksum, old value and new value.
//      Compare with your TODO 9 result. Real routers do it this way. Why?
//
//   C. Layer 2 integrity: Ethernet uses a CRC-32 (the FCS), not the Internet checksum.
//      Research "CRC-32 IEEE 802.3" and implement it bit by bit (reflected polynomial
//      0xEDB88320, initial value 0xFFFFFFFF, final XOR 0xFFFFFFFF).
//      Expected CRC of all 60 bytes: 0x7f2849b4
//      Then: why does CRC catch errors the Internet checksum misses? (Try swapping two
//      16-bit words in the header: does the IP checksum notice? Does the CRC?)
//
//   D. Layer 1: before any frame, Ethernet sends a preamble and a Start Frame Delimiter
//      (search: "Ethernet preamble SFD"). Classic 10 Mbit Ethernet puts bits on the wire
//      with Manchester encoding, and sends each byte LEAST significant bit first.
//      Print the SFD byte 0xD5 as it travels on the wire: first as 8 bits in wire order,
//      then Manchester-encoded (16 half-bit symbols).
//      Check: in wire order, the SFD ends with "11". That's how the receiver knows the
//      frame starts now.
