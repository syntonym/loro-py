from loro import LoroDoc, LoroText, TreeParentId, ValueOrContainer

def test_tree():
    doc = LoroDoc()
    tree = doc.get_tree("tree")
    root = tree.create()
    child = tree.create(TreeParentId.Node(root))
    assert tree.children(TreeParentId.Node(root)) == [child]
    assert tree.children(TreeParentId.Root()) == [root]

    child_meta = tree.get_meta(child)
    text = LoroText()
    text.insert(0, 'hi')
    child_meta.insert_container('a', text)
    child_meta.insert('b', value='basic')

    container = child_meta.get(key='a')
    assert isinstance(container, ValueOrContainer.Container)
    assert isinstance(container.container, LoroText)
    assert container.container.to_string() == 'hi'

    value = child_meta.get(key='b')
    assert isinstance(value, ValueOrContainer.Value)
    assert value.value == 'basic'
