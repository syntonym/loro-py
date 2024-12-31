from loro import LoroDoc

if __name__ == "__main__":
    doc = LoroDoc()
    text = doc.get_text("text")
    sub =  doc.subscribe_root(lambda event: print(event))
    text.insert(0, "abc")
    doc.commit()
    print(text.to_string())
    sub()
