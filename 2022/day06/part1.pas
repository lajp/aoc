program part1;
var ip: string;
var i, ans, k, j: int64;
var c: char;
var cs: array[0..3] of char;
var valid: boolean;

begin
    for i := 0 to 3 do
        read(cs[i]);
    ans := 4;
    while not eoln do
    begin
        valid := true;
        for k := 0 to 3 do
        begin
            for j := 0 to 3 do
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
        for i := 0 to 2 do
            cs[i] := cs[i+1];
        read(cs[3]);
        inc(ans)
    end;
end.
