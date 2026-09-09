import 'package:flutter/material.dart';
import 'package:flutter_btc_wallet/src/rust/api/wrapper.dart';

class BtcWalletPage extends StatefulWidget {
  const BtcWalletPage({super.key, required this.dataPath});

  final String dataPath;

  @override
  State<BtcWalletPage> createState() => _BtcWalletPageState();
}

class _BtcWalletPageState extends State<BtcWalletPage> {
  MutexBtcWallet? _wallet;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          BigCard(balance: BigInt.from(123)),
          SizedBox(height: 10),
          ElevatedButton(
            onPressed: () async {
              final walletPath = '${widget.dataPath}/wallet.db';

              try {
                final wallet = await createWallet(
                  network: 'regtest',
                  electrumServer: 'tcp://192.168.0.41:50001',
                  passphrase: 'abcdefg12345',
                  walletPath: walletPath,
                );
                setState(() {
                  _wallet = wallet;
                });
                debugPrint('wallet created: $walletPath');
              } catch (e) {
                debugPrint('createWallet error: $e');
              }
            },
            child: Text("Create Wallet"),
          ),
          SizedBox(height: 10),
          Text(_wallet != null ? 'Wallet ready' : 'No wallet'),
        ],
      ),
    );
  }
}

class BigCard extends StatelessWidget {
  const BigCard({super.key, required this.balance});

  final BigInt balance;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // ↓ Add this.
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    return Card(
      color: theme.colorScheme.primary,
      child: Padding(
        padding: const EdgeInsets.all(20),
        // ↓ Change this line.
        child: Text(balance.toString(), style: style),
      ),
    );
  }
}
