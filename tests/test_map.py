from loro import LoroDoc, LoroList, ContainerType

def test_map():
    doc = LoroDoc()
    map = doc.get_map("map")
    map.insert("key", "value")
    list = map.insert_container("key2", LoroList())
    list.insert(0, "value2")
    doc.commit()
    print(doc.get_value())
