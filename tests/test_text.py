from loro import LoroDoc, StyleConfigMap


def test_text():
    doc = LoroDoc()
    doc.config_text_style(StyleConfigMap.default_rich_text_config())
    text = doc.get_text("text")
    text.insert(0, "Hello world!")
    text.mark(start=0, end=5, key="bold", value=True)

    deltas = text.to_delta()
    for index, delta in enumerate(deltas):
        if index == 0:
            assert "insert" in delta
            assert delta["insert"] == "Hello"
            assert delta["attributes"] == {"bold": True}
        elif index == 1:
            assert "insert" in delta
            assert delta["insert"] == " world!"
            assert "attributes" not in delta
