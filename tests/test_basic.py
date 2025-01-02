from loro import LoroDoc

def test_basic():
    doc = LoroDoc()
    text = doc.get_text("text")
    text.insert(0, "abc")
    doc.commit()
    assert text.to_string() == "abc"
