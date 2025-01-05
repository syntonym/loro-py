from loro import LoroDoc, ExportMode

def test_basic():
    doc = LoroDoc()
    doc.export(ExportMode.Snapshot())
    text = doc.get_text("text")
    text.insert(0, "abc")
    doc.commit()
    assert text.to_string() == "abc"
