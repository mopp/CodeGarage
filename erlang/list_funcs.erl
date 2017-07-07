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


encode_modified(List) ->
    F = fun([X | T], Acc) ->
            case 1 + length(T) of
                1 -> [X | Acc];
                N -> [{N, X} | Acc]
            end
        end,
    lists:foldr(F, [], pack(List)).


decode(List) ->
    F = fun(X, Acc) ->
            decode(X, []) ++ Acc
        end,
    lists:foldr(F, [], List).
decode({0, _}, Acc) ->
    Acc;
decode({N, A}, Acc) ->
    decode({N - 1, A}, [A | Acc]).


duplicate(List) ->
    reverse(duplicate(List, [])).
duplicate([], Acc) ->
    Acc;
duplicate([X | T], Acc) ->
    duplicate(T, [X, X | Acc]).


duplicate_n(N, List) ->
    reverse(duplicate_n(N, List, [])).
duplicate_n(_, [], Acc) ->
    Acc;
duplicate_n(N, [H | T], Acc) ->
    duplicate_n(N, T, dup(N, H, []) ++ Acc).
dup(0, _, Acc) ->
    Acc;
dup(N, X, Acc) ->
    dup(N - 1, X, [X | Acc]).



drop(N, List) ->
    reverse(drop(N, 1, List, [])).
drop(_, _, [], Acc) ->
    Acc;
drop(N, C, [H | Rest], Acc) ->
    X =
        case C rem N == 0 of
            true  -> Acc;
            false -> [H | Acc]
        end,
    drop(N, C + 1, Rest, X).


split(N, List) ->
    split(N, List, []).
split(0, List, Acc) ->
    {reverse(Acc), List};
split(N, [H | T], Acc) ->
    split(N - 1, T, [H | Acc]).



slice(N, M, List) ->
    reverse(slice(N, M - N, List, [])).
slice(0, 0, _, Acc) ->
    Acc;
slice(0, M, [H | T], Acc) ->
    slice(0, M - 1, T, [H | Acc]);
slice(N, M, [_ | T], Acc) ->
    slice(N - 1, M, T, Acc).

rotate(0, List) ->
    List;
rotate(N, [H | T]) when 0 < N ->
    rotate(N - 1, T ++ [H]);
rotate(N, List) ->
    rotate(len(List) + N, List).


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
    [{4,a}, b, {2,c}, {2,a}, d, {4,e}] = encode_modified([a, a, a, a, b, c, c, a, a, d, e, e, e, e]),
    [a, a, a, a, b, c, c, a, a, d, e, e, e, e] = decode([{4, a}, {1, b}, {2, c}, {2, a}, {1, d}, {4, e}]),
    [a, a, b, b, c, c, c, c, d, d] = duplicate([a, b, c, c, d]),
    [a, a, a, b, b, b, c, c, c, c, c, c, d, d, d] = duplicate_n(3, [a, b, c, c, d]),
    [a, b, d, e, g, h, j, k] = drop(3, [a, b, c, d, e, f, g, h, i, j, k]),
    {[a, b, c], [d, e, f, g, h, i, j, k]} = split(3, [a, b, c, d, e, f, g, h, i, j, k]),
    [d, e, f, g] = slice(3, 7, [a, b, c, d, e, f, g, h, i, j, k]),
    [d, e, f, g, h, i, j, k, a, b, c] = rotate(3, [a, b, c, d, e, f, g, h, i, j, k]),
    [j, k, a, b, c, d, e, f, g, h, i] = rotate(-2, [a, b, c, d, e, f, g, h, i, j, k]),
    ok.
