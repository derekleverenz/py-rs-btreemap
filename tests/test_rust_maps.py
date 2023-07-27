from btree_map import IntBTreeMap, StringBTreeMap


def test_int_map():
    map = IntBTreeMap()
    map[1] = "pierogi"
    map[23] = "gyoza"
    map[-10] = {"name": "coco"}

    assert len(map) == 3
    assert list(map.keys()) == [-10, 1, 23]
    assert list(map.items()) == [(-10, {"name": "coco"}), (1, "pierogi"), (23, "gyoza")]
    assert map.get(9) is None
    assert map[1] == "pierogi"

    map[1] = "sammy"
    assert len(map) == 3
    assert map[1] == "sammy"

    assert list(map.items_final()) == [
        (-10, {"name": "coco"}),
        (1, "sammy"),
        (23, "gyoza"),
    ]
    assert len(map) == 0

    assert map.setdefault(9, 23) == 23
    assert len(map) == 1
    assert map[9] == 23


def test_str_map():
    map = StringBTreeMap()
    map["abc"] = "123"
    map["aaaa"] = "foo"
    map["zzz"] = {1: "bar"}

    assert len(map) == 3
    assert list(map.keys()) == ["aaaa", "abc", "zzz"]
    assert list(map.items()) == [("aaaa", "foo"), ("abc", "123"), ("zzz", {1: "bar"})]
    assert map.get("alskdjf") is None

    map["abc"] = 7
    assert len(map) == 3
    assert map["abc"] == 7

    assert list(map.items_final()) == [("aaaa", "foo"), ("abc", 7), ("zzz", {1: "bar"})]
    assert len(map) == 0

    assert map.setdefault("new", "something") == "something"
    assert len(map) == 1
    assert map["new"] == "something"
