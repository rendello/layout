
import nltk
from collections import Counter


def ngrams_f(file_name, n):
    with open(file_name, 'r') as file:
        c = Counter()
        for line in file:
            if "'" not in line:
                word = line.strip()
                word_ngrams = nltk.ngrams(word, n, pad_left=True, pad_right=True,  left_pad_symbol=' ', right_pad_symbol=' ')
                for ng in word_ngrams:
                    c.update([''.join(ng).strip()])
        return c


def prefixes_f(file_name, n):
    with open(file_name, 'r') as file:
        c = Counter()
        for line in file:
            if "'" not in line:
                word = line.strip()
                c.update([word[:n]])
        return c



def suffixes_f(file_name, n):
    with open(file_name, 'r') as file:
        c = Counter()
        for line in file:
            if "'" not in line:
                word = line.strip()
                c.update([word[-n:]])
        return c


def generate_rules(sequence_function, lengths):
    iu_sequences = Counter()
    en_sequences = Counter()
    fr_sequences = Counter()

    for n in lengths:
        en_sequences += sequence_function("lexicon_english.txt", n)
        fr_sequences += sequence_function("lexicon_french.txt", n)
        iu_sequences += sequence_function("lexicon_inuk.txt", n)

    dissallowed_sequences = set(en_sequences) | set(fr_sequences)

    allowed_sequences = []
    for item, _ in iu_sequences.most_common(1_000_000_000):
        if item not in dissallowed_sequences:
            allowed_sequences.append(item)

    deduped = []
    for seq in allowed_sequences:
        for smaller_seq in deduped:
            if smaller_seq in seq:
                break
        else:
            deduped.append(seq)

    return sorted(sorted([seq for seq in allowed_sequences if seq in deduped][:100]), key=len)


def filter(ngrams, sequences):
    new_sequences = []
    for seq in sequences:
        for ng in ngrams:
            if ng in seq:
                break
        else:
            new_sequences.append(seq)
    return new_sequences


def print_all(*args):
    for arg in args:
        print("="*80)
        for item in arg:
            print(item)



lengths = [1,2,3]

ngrams = generate_rules(ngrams_f, lengths)

prefixes = filter(ngrams, generate_rules(prefixes_f, lengths))
suffixes = filter(ngrams, generate_rules(suffixes_f, lengths))

print_all(ngrams, prefixes, suffixes)



# for ng in ngrams:
#     print(f'(?:{ng})|', end="")