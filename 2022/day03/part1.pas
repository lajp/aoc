program part1;
var ip: string;
var items: array['A'..'z'] of integer;
var l, i, ans: int64;
var item: char;

begin
    ans := 0;
    for item := 'A' to 'z' do
        items[item] := 0;

    while not eof do
    begin
        readln(ip);
        l := length(ip);

        for i := 0 to l div 2 do
        begin
            if items[ip[i]] < 0 then
            begin
                item := ip[i];
                break
            end;
            inc(items[ip[i]]);

            if items[ip[l-i]] > 0 then
            begin
                item := ip[l-i];
                break
            end;
            dec(items[ip[l-i]]);
        end;

        if (item >= 'a') and (item <= 'z') then
            inc(ans, ord(item)-ord('a')+1)
        else
            inc(ans, ord(item)-ord('A')+27);

        for item := 'A' to 'z' do
            items[item] := 0;
    end;
    writeln(ans);
end.
