from loro import LoroDoc

def test_tree():
    doc = LoroDoc()
    tree = doc.get_tree("tree")
    root = tree.create()
    child = tree.create(root)
    assert tree.children(root) == [child]