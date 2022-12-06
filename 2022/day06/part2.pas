program part2;
var ip: string;
var i, ans, k, j: int64;
var c: char;
var cs: array[0..13] of char;
var valid: boolean;

begin
    for i := 0 to 13 do
        read(cs[i]);
    ans := 14;
    while not eoln do
    begin
        valid := true;
        for k := 0 to 13 do
        begin
            for j := 0 to 13 do
            begin
                if j = k then
                    continue;
                if cs[j] = cs[k] then
                    valid := false;
            end;
        end;
        if valid then
        begin
            writeln(ans);
            break
        end;
        for i := 0 to 12 do
            cs[i] := cs[i+1];
        read(cs[13]);
        inc(ans)
    end;
end.
