program part2;
var ans: int64;
var op, res: char;
var ip: string;

begin
    ans := 0;
    while not eof do
    begin
        readln(ip);
        op := ip[1]; { Why tf is this [1] and not [0]??! }
        res := ip[3];
        case (res) of
            'X' : begin
                    if op = 'A' then
                        inc(ans, 3)
                    else
                        inc(ans, ord(op)-1-64);
                end;
            'Y' : inc(ans, 3+ord(op)-64);
            'Z' : begin
                if op = 'C' then
                    inc(ans, 7)
                else
                    inc(ans, 6+ord(op)+1-64);
                end;
        end;
    end;
    writeln(ans);
end.
