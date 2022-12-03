program part2;
var ip: string;
var items: array['A'..'z'] of shortint;
var i, ans, e: int64;
var item: char;

begin
    ans := 0;
    for item := 'A' to 'z' do
        items[item] := 0;

    while not eof do
    begin
        for e := 0 to 2 do
        begin
            readln(ip);
            for i := 0 to length(ip) do
            begin
                items[ip[i]] := items[ip[i]] or (1 << e);
            end;
        end;

        for item := 'A' to 'z' do
            if items[item] = %111 then
                break;

        if (item >= 'a') and (item <= 'z') then
            inc(ans, ord(item)-ord('a')+1)
        else
            inc(ans, ord(item)-ord('A')+27);

        for item := 'A' to 'z' do
            items[item] := 0;
    end;
    writeln(ans);
end.
