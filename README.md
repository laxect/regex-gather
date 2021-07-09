# regex-gather
gather regex from input and some utils

```
# gather regex
input:
  ["[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4", "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 02 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4"]
out:
  \[Lilith\-Raws\] 桃子男孩渡海而来 / Peach Boy Riverside \- (?P<ep>\d+) \[Baha\]\[WEB\-DL\]\[1080p\]\[AVC AAC\]\[CHT\]\.mp4

# gather template
input:
  ["[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4", "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 02 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4"]
out:
  [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - {ep} [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4

# guess from one
input:
  [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4
out:
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - (?P<ep>\d+) \[Baha\]\[WEB\-DL\]\[1080p\]\[AVC AAC\]\[CHT\]\.mp4
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][p][AVC AAC][CHT].mp10804

 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 \[Baha\]\[WEB\-DL\]\[(?P<ep>\d+)p\]\[AVC AAC\]\[CHT\]\.mp4
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4

 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 \[Baha\]\[WEB\-DL\]\[1080p\]\[AVC AAC\]\[CHT\]\.mp(?P<ep>\d+)
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4

# guess template
input:
  [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4
out:
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - {ep} [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][p][AVC AAC][CHT].mp10804

 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][{ep}p][AVC AAC][CHT].mp4
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4

 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp{ep}
 - [Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4


```
