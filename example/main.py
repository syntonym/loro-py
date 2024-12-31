from loro import LoroDoc

if __name__ == "__main__":
    doc = LoroDoc()
    text = doc.get_text("text")
    map = doc.get_map("map")
    sub =  doc.subscribe_root(lambda event: print(event))
    text.insert(0, "abc")
    map.insert("key", "value")
    doc.commit()
    print(text.to_string())
    print(map.get_value())
    sub()

