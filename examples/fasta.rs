use p_arse::{any, eoi, CharExt, Parser};

fn main() {
    let nl = '\n';

    let header = {
        let parse_header = |(_, header, _): (_, Vec<(_, char)>, _)| {
            header.iter().map(|((), c)| c).collect::<String>()
        };

        let header_content = (nl.not_ahead(), any()).more();
        let header_tag = ">";

        (header_tag, header_content, nl).map(parse_header)
    };

    let sequence = {
        let concat_subsequence = |cs: Vec<char>| cs.iter().collect::<String>();
        let parse_sequence =
            |(first, tail, _nls): (String, Vec<(_, String)>, _)| {
                let mut sequence = first;
                tail.into_iter().for_each(|(_nl, subsequence)| {
                    sequence.push_str(&subsequence)
                });

                sequence
            };
        let sequence_char = ('A'.to('Z')).or('*').or('-');
        let subsequence = sequence_char.more().map(concat_subsequence);

        (subsequence, (nl, subsequence).zore(), nl.zore()).map(parse_sequence)
    };

    let entry = (header, sequence);

    let file = {
        let parse_file = |(_, entries, _)| entries;

        (nl.zore(), entry.zore(), eoi()).map(parse_file)
    };

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
