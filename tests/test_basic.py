from loro import LoroDoc, ExportMode, VersionVector

def test_basic():
    doc = LoroDoc()
    doc.export(ExportMode.Snapshot())
    j = doc.export_json_updates(VersionVector(), VersionVector())
    doc.import_json_updates(j)
    text = doc.get_text("text")
    text.insert(0, "abc")
    doc.commit()
    assert text.to_string() == "abc"

def test_version_vector():
    doc1 = LoroDoc()
    assert doc1.state_vv == VersionVector()
    text = doc1.get_text("text")
    text.insert(0, "abc")

    snapshot = doc1.export(ExportMode.Snapshot())
    doc2 = LoroDoc()
    assert doc1.state_vv != doc2.state_vv

    doc2.import_(snapshot)
    assert doc2.get_text("text").to_string() == "abc"
    assert doc1.state_vv == doc2.state_vv
