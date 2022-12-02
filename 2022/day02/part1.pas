program part1;
var ans: int64;
var me, op: char;
var ip: string;

begin
    ans := 0;
    while not eof do
    begin
        readln(ip);
        op := ip[1]; { Why tf is this [1] and not [0]??! }
        me := chr(ord(ip[3])-23);
        if op = me then
            inc(ans, 3+ord(me)-64)
        else if ((ord(op)+1) = ord(me)) or ((op = 'C') and (me = 'A')) then
            inc(ans, 6+ord(me)-64)
        else
            inc(ans, ord(me)-64);
    end;
    writeln(ans);
end.
