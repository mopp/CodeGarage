-module(list_funcs).
-compile(export_all).

last([]) ->
    undefined;
last([X]) ->
    X;
last([_ | T]) ->
    last(T).

penultimate([]) ->
    undefined;
penultimate([_]) ->
    undefined;
penultimate([X, _ | []]) ->
    X;
penultimate([_, Y | T]) ->
    penultimate([Y | T]).

nth(_, []) ->
    undefined;
nth(0, [H | _]) ->
    H;
nth(N, [_ | T]) ->
    nth(N - 1, T).

len(List) ->
    len(0, List).
len(N, []) ->
    N;
len(N, [_ | T]) ->
    len(N + 1, T).

reverse(List) ->
    reverse(List, []).
reverse([], Acc) ->
    Acc;
reverse([H | T], Acc) ->
    reverse(T, [H | Acc]).

is_palindrome(List) ->
    List =:= reverse(List).

flatten(ListList) ->
    flatten(ListList, []).
flatten([], Acc) ->
    Acc;
flatten([H | T], Acc) ->
    flatten(T, Acc ++ H).

% compress(List('a, 'a, 'a, 'a, 'b, 'c, 'c, 'a, 'a, 'd, 'e, 'e, 'e, 'e))
compress([H | T]) ->
    reverse(compress(T, [H])).
compress([], Acc) ->
    Acc;
compress([X | Tx], [X | Ta]) ->
    compress(Tx, [X | Ta]);
compress([X | Tx], [Y | Ta]) ->
    compress(Tx, [X, Y | Ta]).

pack([H | T]) ->
    reverse(pack(T, [[H]])).
pack([], Acc) ->
    Acc;
pack([H | Tx], [[H | T] | Rest]) ->
    pack(Tx, [[H, H | T] | Rest]);
pack([X | Tx], [[H | T] | Rest]) ->
    pack(Tx, [[X], [H | T] | Rest]).

encode([H | T]) ->
    reverse(encode(T, [{1, H}])).
encode([], Acc) ->
    Acc;
encode([H | T], [{N, H} | Rest]) ->
    encode(T, [{N + 1, H} | Rest]);
encode([H | T], [{N, X} | Rest]) ->
    encode(T, [{1, H}, {N, X} | Rest]).




test() ->
    4 = last([1, 2, 3, 4]),
    3 = penultimate([1, 2, 3, 4]),
    3 = nth(2, [1, 2, 3, 4]),
    4 = len([1, 2, 3, 4]),
    [4, 3, 2, 1] = reverse([1, 2, 3, 4]),
    true = is_palindrome([1, 2, 3, 2, 1]),
    false = is_palindrome([2, 3, 2, 1]),
    [1, 2, 3, 4, 5, 6] = flatten([[1, 2, 3], [4, 5], [6]]),
    [a, b, c, a, d, e] = compress([a, a, a, a, b, c, c, a, a, d, e, e, e, e]),
    [[a, a, a, a], [b], [c, c], [a, a], [d], [e, e, e, e]] = pack([a, a, a, a, b, c, c, a, a, d, e, e, e, e]),
    [{4,a}, {1,b}, {2,c}, {2,a}, {1,d}, {4,e}] = encode([a, a, a, a, b, c, c, a, a, d, e, e, e, e]),
    ok.
