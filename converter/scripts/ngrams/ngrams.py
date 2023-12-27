
import nltk
from collections import Counter


def lang_ngrams(file_name, n):
    with open(file_name, 'r') as file:
        c = Counter()
        for line in file:
            word = line.strip()
            word_ngrams = nltk.ngrams(word, n, pad_left=True, pad_right=True,  left_pad_symbol=' ', right_pad_symbol=' ')
            for ng in word_ngrams:
                c.update([''.join(ng).strip()])
        return c


iu_ngrams = Counter()
en_ngrams = Counter()
fr_ngrams = Counter()

for n in [1,2,3,4,5]:
    en_ngrams += lang_ngrams("lexicon_english.txt", n)
    fr_ngrams += lang_ngrams("lexicon_french.txt", n)
    iu_ngrams += lang_ngrams("lexicon_inuk.txt", n)


dissallowed_ngrams = set(en_ngrams) | set(fr_ngrams)

allowed_ngrams = []
for item, _ in iu_ngrams.most_common(1_000_000_000):
    if item not in dissallowed_ngrams:
        allowed_ngrams.append(item)


deduped = []
for ng in sorted(sorted(allowed_ngrams), key=len):
    for smaller_ng in deduped:
        if smaller_ng in ng:
            break
    else:
        deduped.append(ng)

ngrams = [ng for ng in allowed_ngrams if ng in deduped]
for ng in ngrams[:100]:
    print(f'(?:{ng})|', end="")