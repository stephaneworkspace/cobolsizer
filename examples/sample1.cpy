      ******************************************************************
      *
      *    TODO:
      *    - REDEFINES
      *    - Z
      *    - COMP-1
      *    - 9.9 
      *    - Z.Z 
      *    - FILLER
      *
      ******************************************************************
       01  STRUCT.
           03 STRUCT-TEXTE                       PIC X(60) VALUE
           "Message de test".
           03 STRUCT-I                           PIC 9999.
           03 STRUCT-J                           PIC 9(6).
           03 STRUCT-BUFFER-1024                 PIC X(1024).
      *    03 STRUCT-RETURNCODE                  PIC S9(4) BINARY.
      *-----------------------------------------------------------------
      *    UN COMMENTAIRE
      *-----------------------------------------------------------------
           03 STRUCT-ARRAY OCCURS 10.
               05 STRUCT-ARRAY-NO                PIC 99.
               05 STRUCT-ARRAY-NOM               PIC X(100).
           03 STRUCT-NEXT                        PIC 9.
