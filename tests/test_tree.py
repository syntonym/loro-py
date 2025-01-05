from loro import LoroDoc, TreeParentId

def test_tree():
    doc = LoroDoc()
    tree = doc.get_tree("tree")
    root = tree.create()
    child = tree.create(TreeParentId.Node(root))
    assert tree.children(TreeParentId.Node(root)) == [child]