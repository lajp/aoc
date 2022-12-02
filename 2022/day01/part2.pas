program part1;
var n, cur, i, code, ans: int64;
var ip: string;
var q: array[0..2] of int64;

procedure tryadd(n: int64);
begin
    if n > q[0] then
    begin
        q[2] := q[1];
        q[1] := q[0];
        q[0] := n;
    end
    else if n > q[1] then
    begin
        q[2] := q[1];
        q[1] := n
    end
    else if n > q[2] then
    begin
        q[2] := n;
    end;
end;

begin
    cur := 0;
    for i := 0 to 2 do
        q[i] := 0;

    repeat
    begin
        readln(ip);
        if ip = '' then
        begin
            tryadd(cur);
            cur := 0;
            continue
        end;
        val(ip, n, code);
        inc(cur, n);
    end
    until eof;

    tryadd(cur);

    ans := 0;
    for i := 0 to 2 do
        inc(ans, q[i]);

    writeln(ans);
end.
