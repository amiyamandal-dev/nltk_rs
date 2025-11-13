import re

import pytest

from nltk.tokenize.regexp import (
    BlanklineTokenizer,
    RegexpTokenizer,
    WordPunctTokenizer,
    blankline_tokenize,
    regexp_tokenize,
    regexp_tokenize_batch,
    wordpunct_tokenize,
)


@pytest.mark.parametrize(
    "pattern,text",
    [
        (r"\\w+|[^\\w\\s]+", "Good muffins cost $3.88 in New York."),
        (r"[A-Za-z]+", "NLTK's rust tokenizer"),
    ],
)
def test_regexp_tokenize_matches_python(pattern, text):
    expected = RegexpTokenizer(pattern).tokenize(text)
    assert regexp_tokenize(text, pattern) == expected


def test_regexp_tokenize_gaps_with_discard():
    text = "Good muffins\n\nPlease buy me two.\n\nThanks."
    pattern = r"\s+"
    expected = RegexpTokenizer(pattern, gaps=True).tokenize(text)
    assert regexp_tokenize(text, pattern, gaps=True) == expected


def test_regexp_tokenize_batch_parallel_matches_python():
    texts = ["One. Two.", "Three!", "Four? Five."]
    pattern = r"\\w+|[^\\w\\s]+"
    expected = [RegexpTokenizer(pattern).tokenize(t) for t in texts]
    assert regexp_tokenize_batch(texts, pattern) == expected


def test_regexp_tokenize_locale_flag_matches_python_error():
    text = "über-cool"
    pattern = r"[a-z]+"

    with pytest.raises(ValueError, match="cannot use LOCALE"):
        RegexpTokenizer(pattern, flags=re.LOCALE).tokenize(text)

    with pytest.raises(ValueError, match="cannot use LOCALE"):
        regexp_tokenize(text, pattern, flags=re.LOCALE)

    with pytest.raises(ValueError, match="cannot use LOCALE"):
        regexp_tokenize_batch([text], pattern, flags=re.LOCALE)


def test_wordpunct_tokenize_matches_python():
    text = "Good muffins cost $3.88 in New York."
    expected = WordPunctTokenizer().tokenize(text)
    assert wordpunct_tokenize(text) == expected


def test_blankline_tokenize_matches_python():
    text = "Line one.\n\nLine two.\n\n\nLine three."
    expected = BlanklineTokenizer().tokenize(text)
    assert blankline_tokenize(text) == expected
