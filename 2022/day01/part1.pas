program part1;
var n, max, cur, code: int64;
var ip: string;

begin
    max := 0;
    cur := 0;
    repeat
    begin
        readln(ip);
        if ip = '' then
        begin
            if cur > max then
                max := cur;
            cur := 0;
            continue
        end;
        val(ip, n, code);
        inc(cur, n);
    end
    until eof;

    if cur > max then
        max := cur;
    writeln(max);
end.
