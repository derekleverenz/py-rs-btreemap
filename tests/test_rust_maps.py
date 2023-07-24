from btree_map import IntBTreeMap


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
