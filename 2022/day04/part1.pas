program part1;
uses sysutils;
var a1, a2, b1, b2, ans: int64;
var ip: string;

begin
    ans := 0;
    while not eof do
    begin
        readln(ip);
        sscanf(ip, '%d-%d,%d-%d', [@a1, @a2, @b1, @b2]);
        if ((a1 >= b1) and (a2 <= b2)) or ((b1 >= a1) and (b2 <= a2)) then
            inc(ans);
    end;
    writeln(ans);
end.
