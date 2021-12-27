use p_arse::{any, eoi, CharExt, Parser, TupleExt};

fn main() {
    let nl = '\n';

    let header = {
        let header_content =
            (nl.not_ahead(), any()).r0().more().maps(|s| s.to_string());
        let header_tag = ">";
        let header = (header_tag, header_content, nl).r2().r0();

        header
    };

    let sequence = {
        let parse_sequence = |(first, tail): (String, Vec<String>)| {
            let mut sequence = first;
            tail.into_iter()
                .for_each(|subsequence| sequence.push_str(&subsequence));

            sequence
        };

        let sequence_char = ('A'.to('Z')).or('*').or('-');
        let subsequence = sequence_char.more().maps(|s| s.to_string());
        let sequence = (subsequence, (nl, subsequence).r0().zore(), nl.zore())
            .r2()
            .map(parse_sequence);

        sequence
    };

    let entry = (header, sequence);
    let file = (nl.zore(), entry.zore(), eoi()).r2().r0();

    // `\` at the end of the line in string means 'ignore following whitespace'.
    let fasta = "\
    >MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken\n\
    MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID\n\
    FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA\n\
    DIDGDGQVNYEEFVQMMTAK*\n\
    >gi|5524211|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]\n\
    LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV\n\
    EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG\n\
    LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL\n\
    GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX\n\
    IENY\n\
    ";

    let (fasta, _) = file.p_arse(fasta).unwrap();
    dbg!(fasta);
}
