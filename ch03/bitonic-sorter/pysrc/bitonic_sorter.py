u"Bitonic Merge Sort モジュール"

# Pythonによる sort 関数


def sort(x, up):
    """
    リストxの要素をupで指定された向きにソートする。
    up が True なら昇順、False なら降順になる。
    x の要素数は2のべき乗でなければならない
    （さもなければソート結果がおかしくなる）
    """
    if len(x) <= 1:
        # 要素数が1になったら終わり
        return x
    else:
        # ステップ 1a
        # リストの前半(first)は昇順、後半(second)は降順でソートする。
        mid_point = len(x) // 2
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        # ステップ 1b
        # 2分割したリストを1つに結合する
        x1 = first + second

        # ステップ 2: サブソートに進む
        return _sub_sort(x1, up)


def _sub_sort(x, up):
    """
    バイトニックにソートされたリストxの前半と後半を、up で指定された向きに、
    比較、交換し、前半と後半それぞれについて再帰的にサブソートを適用する
    """

    if len(x) == 1:
        # 要素数が1になったらおわり
        return x
    else:
        # ステップ2a
        # 要素数nのバイトニック列の要素をn/2要素おきに比較して、
        # up で指定された順序 (昇順または降順) になるように変換する
        _compare_and_swap(x, up)

        # ステップ2b
        # データ列を半分に分割し、それぞれに対して_sub_sortを繰り返す
        mid_point = len(x) // 2
        first = _sub_sort(x[:mid_point], up)
        second = _sub_sort(x[mid_point:], up)

        # ステップ2c
        # 2分割したデータ列を1つに結合する
        return first + second


def _compare_and_swap(x, up):
    """
    要素数 n のバイトニック列の要素を n/2 要素おきに比較して、
    up で指定された順序 (昇順 or 降順) になるように交換する(ステップ2a)
    """
    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            # 要素を交換
            x[i], x[mid_point + i] = x[mid_point + i], x[i]
